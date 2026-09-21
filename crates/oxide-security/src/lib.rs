use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRecord {
    pub key_hash: String,
    pub name: String,
    pub created_at: i64,
    pub max_tpm: u32,
}

#[derive(Debug, Clone)]
pub struct TokenBucket {
    pub capacity: u32,
    pub tokens: f32,
    pub last_refill: i64,
}

impl TokenBucket {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            tokens: capacity as f32,
            last_refill: chrono::Utc::now().timestamp(),
        }
    }

    pub fn try_consume(&mut self, amount: u32) -> bool {
        let now = chrono::Utc::now().timestamp();
        let elapsed = (now - self.last_refill).max(0) as f32;
        // Refill tokens over time (tokens per minute rate)
        let refill_rate = (self.capacity as f32) / 60.0;
        self.tokens = (self.tokens + elapsed * refill_rate).min(self.capacity as f32);
        self.last_refill = now;

        if self.tokens >= amount as f32 {
            self.tokens -= amount as f32;
            true
        } else {
            false
        }
    }
}

pub struct SecurityManager {
    db: Surreal<Any>,
    rate_limiters: DashMap<String, TokenBucket>,
}

impl SecurityManager {
    pub fn new(db: Surreal<Any>) -> Arc<Self> {
        Arc::new(Self {
            db,
            rate_limiters: DashMap::new(),
        })
    }

    pub fn db(&self) -> &Surreal<Any> {
        &self.db
    }


    pub fn hash_key(raw_key: &str) -> Result<String, String> {
        let hash = blake3::hash(raw_key.as_bytes());
        Ok(hash.to_hex().to_string())
    }

    pub fn verify_key(raw_key: &str, hash: &str) -> bool {
        let computed = blake3::hash(raw_key.as_bytes()).to_hex().to_string();
        computed == hash
    }

    pub async fn check_rate_limit(&self, key_id: &str, max_tpm: u32, tokens: u32) -> bool {
        let mut bucket = self
            .rate_limiters
            .entry(key_id.to_string())
            .or_insert_with(|| TokenBucket::new(max_tpm));
        bucket.try_consume(tokens)
    }
}
