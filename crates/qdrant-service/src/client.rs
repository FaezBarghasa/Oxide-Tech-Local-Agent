use qdrant_client::Qdrant;
use std::env;
use moka::future::Cache;
use std::time::Duration;

pub struct QdrantServiceClient {
    pub client: Qdrant,
    pub l1_semantic_cache: Cache<String, Vec<f32>>,
}

impl QdrantServiceClient {
    pub fn new() -> Result<Self, anyhow::Error> {
        let qdrant_url = env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6334".to_string());
        
        let client = Qdrant::from_url(&qdrant_url).build()?;
        let l1_semantic_cache = Cache::builder()
            .max_capacity(16384) 
            .time_to_live(Duration::from_secs(3600)) 
            .build();

        Ok(Self { client, l1_semantic_cache })
    }

    pub async fn health_check(&self) -> Result<(), anyhow::Error> {
        let _ = self.client.list_collections().await?;
        Ok(())
    }
}

