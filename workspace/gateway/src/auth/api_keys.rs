use sha2::{Sha256, Digest};
use std::sync::Arc;
use memory::SurrealClient;
use tracing::{info, warn};

pub struct ApiKeyManager {
    pub db: Option<Arc<SurrealClient>>,
}

impl ApiKeyManager {
    pub fn new(db: Option<Arc<SurrealClient>>) -> Self {
        Self { db }
    }

    pub fn hash_key(&self, raw_key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw_key.as_bytes());
        let result = hasher.finalize();
        let mut hashed_str = String::new();
        for byte in result {
            hashed_str.push_str(&format!("{:02x}", byte));
        }
        hashed_str
    }

    pub async fn validate_key(&self, raw_key: &str) -> Result<bool, anyhow::Error> {
        let hashed = self.hash_key(raw_key);
        
        let Some(ref client) = self.db else {
            warn!("SurrealDB not initialized; API key validation skipped/mocked.");
            return Ok(raw_key == "mock-developer-key");
        };

        // Query the api_key table for matching key_hash
        let mut response = client.db
            .query("SELECT * FROM api_key WHERE key_hash = $hash LIMIT 1")
            .bind(("hash", hashed))
            .await?;

        let keys: Vec<serde_json::Value> = response.take(0)?;
        if keys.is_empty() {
            return Ok(false);
        }

        // Check expiration
        if let Some(key) = keys.first() {
            if let Some(expires_at_val) = key.get("expires_at") {
                if let Some(expires_str) = expires_at_val.as_str() {
                    if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(expires_str) {
                        if expires < chrono::Utc::now() {
                            info!("API key expired.");
                            return Ok(false);
                        }
                    }
                }
            }
        }

        Ok(true)
    }
}
