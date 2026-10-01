//! Embedded In-Process SurrealDB Engine
//!
//! Provides zero-network, daemon-free database storage using `surrealdb::engine::local::SurrealKV`
//! (with automatic fallback to `surrealdb::engine::local::Mem` for memory or test environments).
//! Embeds all migration `.surql` scripts directly into the binary `.rodata` and runs them on init.

use surrealdb::engine::any::connect;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use std::path::Path;
use tracing::info;

pub const MIGRATION_001: &str = include_str!("migrations/001_init_schema.surql");
pub const MIGRATION_002: &str = include_str!("migrations/002_cloud_training.surql");
pub const MIGRATION_003: &str = include_str!("migrations/003_multi_tenant.surql");
pub const MIGRATION_004: &str = include_str!("migrations/004_cloud_training_capture.surql");

use std::sync::Arc;
use tokio::sync::OnceCell;

static GLOBAL_SURREAL_INSTANCE: OnceCell<Arc<EmbeddedSurrealDb>> = OnceCell::const_new();

#[derive(Clone)]
pub struct EmbeddedSurrealDb {
    pub db: Surreal<Any>,
    pub namespace: String,
    pub database: String,
}

impl EmbeddedSurrealDb {
    /// Retrieve or initialize a process-wide thread-safe singleton instance
    pub async fn shared_in_memory() -> Result<Arc<Self>, surrealdb::Error> {
        GLOBAL_SURREAL_INSTANCE
            .get_or_try_init(|| async {
                Self::in_memory().await.map(Arc::new)
            })
            .await
            .cloned()
    }

    /// Connect to an in-memory database instance (ideal for transient sessions & testing)
    pub async fn in_memory() -> Result<Self, surrealdb::Error> {
        Self::connect_url("mem://").await
    }

    /// Connect to an in-process SurrealKV persistent file storage
    pub async fn persistent<P: AsRef<Path>>(data_dir: P) -> Result<Self, surrealdb::Error> {
        let p = data_dir.as_ref();
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let url = format!("surrealkv://{}", p.display());
        Self::connect_url(&url).await
    }

    /// Connect to any local engine URL (surrealkv://, mem://, etc.) and apply embedded migrations
    pub async fn connect_url(url: &str) -> Result<Self, surrealdb::Error> {
        let db = connect(url).await?;
        db.use_ns("workspace").use_db("main").await?;

        let instance = Self {
            db,
            namespace: "workspace".to_string(),
            database: "main".to_string(),
        };

        instance.apply_embedded_migrations().await?;
        Ok(instance)
    }

    /// Executes all embedded .surql migration scripts sequentially
    pub async fn apply_embedded_migrations(&self) -> Result<(), surrealdb::Error> {
        let migrations = [
            ("001_init_schema", MIGRATION_001),
            ("002_cloud_training", MIGRATION_002),
            ("003_multi_tenant", MIGRATION_003),
            ("004_cloud_training_capture", MIGRATION_004),
        ];

        for (name, script) in migrations {
            info!("SurrealDB Embedded: Applying migration '{}'", name);
            let _ = self.db.query(script).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedded_surreal_lifecycle() {
        let db = EmbeddedSurrealDb::in_memory().await.unwrap();
        let res = db.db.query("CREATE user:test SET name = 'Faez', role = 'Architect'").await;
        assert!(res.is_ok());
    }
}
