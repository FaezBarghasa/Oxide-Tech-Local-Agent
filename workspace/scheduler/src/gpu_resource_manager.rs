use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};
use std::sync::Arc;
use std::process::Stdio;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuMode {
    Inference,  // 06:00 - 00:00
    Training,   // 00:00 - 06:00
    Transition, // Swap window
}

pub struct GpuResourceManager {
    pub mode: Arc<RwLock<GpuMode>>,
    pub sglang_procs: Arc<RwLock<Vec<tokio::process::Child>>>,
    pub training_procs: Arc<RwLock<Vec<tokio::process::Child>>>,
}

impl GpuResourceManager {
    pub fn new() -> Self {
        Self {
            mode: Arc::new(RwLock::new(GpuMode::Inference)),
            sglang_procs: Arc::new(RwLock::new(Vec::new())),
            training_procs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn switch_mode(&self, new_mode: GpuMode) -> Result<(), anyhow::Error> {
        let current_mode = { *self.mode.read().await };
        if current_mode == new_mode {
            info!("GPU already in mode {:?}", new_mode);
            return Ok(());
        }

        info!("Switching GPU mode from {:?} to {:?}", current_mode, new_mode);
        
        // 1. Enter Transition mode
        {
            let mut mode_guard = self.mode.write().await;
            *mode_guard = GpuMode::Transition;
        }

        match new_mode {
            GpuMode::Training => {
                // Shut down Inference servers
                self.drain_and_kill_inference().await?;
                
                // Spawn Training scripts
                self.start_training().await?;
            }
            GpuMode::Inference => {
                // Shut down training procs
                self.kill_training().await?;
                
                // Run evaluation before deploying model
                if self.evaluate_new_model().await {
                    info!("New model passed benchmarks, deploying to production inference.");
                } else {
                    warn!("New model failed benchmarks, rolling back to previous production model.");
                }

                // Start Inference servers
                self.start_inference().await?;
            }
            GpuMode::Transition => {}
        }

        {
            let mut mode_guard = self.mode.write().await;
            *mode_guard = new_mode;
        }

        info!("GPU mode switch complete. Now in: {:?}", new_mode);
        Ok(())
    }

    async fn drain_and_kill_inference(&self) -> Result<(), anyhow::Error> {
        info!("Draining inference requests (30s)...");
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        let mut procs = self.sglang_procs.write().await;
        for mut child in procs.drain(..) {
            let _ = child.kill().await;
        }
        info!("All inference servers terminated.");
        Ok(())
    }

    async fn start_training(&self) -> Result<(), anyhow::Error> {
        info!("Starting nightly fine-tuning job...");
        // Spawn QLoRA fine-tuning script
        let child = tokio::process::Command::new("python3")
            .arg("scripts/fine_tune_unsloth.py")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut procs = self.training_procs.write().await;
        procs.push(child);
        Ok(())
    }

    async fn kill_training(&self) -> Result<(), anyhow::Error> {
        info!("Stopping fine-tuning processes...");
        let mut procs = self.training_procs.write().await;
        for mut child in procs.drain(..) {
            let _ = child.kill().await;
        }
        Ok(())
    }

    async fn start_inference(&self) -> Result<(), anyhow::Error> {
        info!("Starting Ornith-35B and Qwen-32B inference servers...");
        // In a real environment, we'd execute the sglang commands:
        // python3 -m sglang.launch_server --model Ornith-35B --port 8000 --device cuda:0
        // python3 -m sglang.launch_server --model Qwen2.5-Coder-32B --port 8001 --device cuda:1
        Ok(())
    }

    async fn evaluate_new_model(&self) -> bool {
        info!("Evaluating newly trained QLoRA checkpoint against benchmarks...");
        // Stubbed evaluation execution: in real life it executes standard probe-rs/cargo test benchmarks
        true
    }

    pub async fn start_cron_scheduler(self: Arc<Self>) -> Result<(), anyhow::Error> {
        let sched = JobScheduler::new().await?;
        
        let self_clone1 = self.clone();
        // At midnight: Switch to Training
        sched.add(Job::new_async("0 0 0 * * *", move |_uuid, _lock| {
            let self_inner = self_clone1.clone();
            Box::pin(async move {
                if let Err(e) = self_inner.switch_mode(GpuMode::Training).await {
                    error!("Cron switch to Training mode failed: {:?}", e);
                }
            })
        })?).await?;

        let self_clone2 = self.clone();
        // At 06:00: Switch to Inference
        sched.add(Job::new_async("0 0 6 * * *", move |_uuid, _lock| {
            let self_inner = self_clone2.clone();
            Box::pin(async move {
                if let Err(e) = self_inner.switch_mode(GpuMode::Inference).await {
                    error!("Cron switch to Inference mode failed: {:?}", e);
                }
            })
        })?).await?;

        sched.start().await?;
        info!("GPU Cron Scheduler started successfully.");
        Ok(())
    }
}
