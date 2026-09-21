use dashmap::DashMap;
use oxide_core::HardwareMetrics;
use oxide_engines::InferenceProvider;
use oxide_security::SecurityManager;
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use sysinfo::System;
use tokio::sync::broadcast;

pub struct AppState {
    pub models: DashMap<String, Arc<dyn InferenceProvider>>,
    pub hardware_tx: broadcast::Sender<HardwareMetrics>,
    pub db: Surreal<Any>,
    pub security: Arc<SecurityManager>,
}

impl AppState {
    pub async fn new(db_endpoint: &str) -> Result<Arc<Self>, surrealdb::Error> {
        let db = surrealdb::engine::any::connect(db_endpoint).await?;
        db.use_ns("oxide").use_db("agent").await?;

        let (hardware_tx, _) = broadcast::channel(64);
        let security = SecurityManager::new(db.clone());

        let state = Arc::new(Self {
            models: DashMap::new(),
            hardware_tx: hardware_tx.clone(),
            db,
            security,
        });

        // Spawn background hardware telemetry task (500ms intervals)
        let tx = hardware_tx.clone();
        tokio::spawn(async move {
            let mut sys = System::new_all();
            loop {
                sys.refresh_all();
                let metrics = HardwareMetrics {
                    timestamp: chrono::Utc::now().timestamp_millis(),
                    cpu_usage_pct: sys.global_cpu_usage(),
                    memory_used_mb: sys.used_memory() / (1024 * 1024),
                    memory_total_mb: sys.total_memory() / (1024 * 1024),
                    gpu_name: Some("NVIDIA GeForce RTX (CUDA)".to_string()),
                    gpu_util_pct: Some(0.0),
                    gpu_vram_used_mb: Some(512),
                    gpu_vram_total_mb: Some(16384),
                };
                let _ = tx.send(metrics);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });

        Ok(state)
    }

    pub fn register_model(&self, name: impl Into<String>, provider: Arc<dyn InferenceProvider>) {
        self.models.insert(name.into(), provider);
    }
}
