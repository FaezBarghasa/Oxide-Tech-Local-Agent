use common::error::{EiosError, Result};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::engine::any::connect;

#[derive(Clone)]
pub struct SurrealClient {
    pub db: Surreal<Any>,
}

impl SurrealClient {
    pub async fn new() -> Result<Self> {
        let db_url = std::env::var("SURREALDB_URL").unwrap_or_else(|_| "mem://".to_string());

        let db = connect(&db_url)
            .await
            .map_err(|e| EiosError::Database(e.to_string()))?;
        db.use_ns("eios")
            .use_db("main")
            .await
            .map_err(|e| EiosError::Database(e.to_string()))?;
        Ok(Self { db })
    }

    pub async fn new_with_url(url: &str) -> Result<Self> {
        let db = connect(url)
            .await
            .map_err(|e| EiosError::Database(e.to_string()))?;
        db.use_ns("eios")
            .use_db("main")
            .await
            .map_err(|e| EiosError::Database(e.to_string()))?;
        Ok(Self { db })
    }
}
