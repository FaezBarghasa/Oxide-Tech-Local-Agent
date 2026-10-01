//! Embedded In-Process Vector Store
//!
//! Replaces external gRPC vector databases with a local, zero-network,
//! thread-safe vector similarity index storing high-dimensional embeddings.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: String,
    pub collection: String,
    pub vector: Vec<f32>,
    pub payload: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredPoint {
    pub id: String,
    pub score: f32,
    pub payload: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorCollection {
    pub name: String,
    pub dimension: usize,
    pub documents: HashMap<String, VectorDocument>,
}

pub struct VectorStore {
    collections: HashMap<String, VectorCollection>,
    storage_path: Option<PathBuf>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
            storage_path: None,
        }
    }

    pub fn with_storage_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            collections: HashMap::new(),
            storage_path: Some(path.as_ref().to_path_buf()),
        }
    }

    /// Ensure collection exists
    pub fn create_collection(&mut self, name: &str, dimension: usize) {
        self.collections.entry(name.to_string()).or_insert_with(|| VectorCollection {
            name: name.to_string(),
            dimension,
            documents: HashMap::new(),
        });
    }

    /// Insert or update vector point
    pub fn upsert(
        &mut self,
        collection: &str,
        id: &str,
        vector: Vec<f32>,
        payload: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let coll = self.collections.entry(collection.to_string()).or_insert_with(|| VectorCollection {
            name: collection.to_string(),
            dimension: vector.len(),
            documents: HashMap::new(),
        });

        coll.documents.insert(
            id.to_string(),
            VectorDocument {
                id: id.to_string(),
                collection: collection.to_string(),
                vector,
                payload,
            },
        );

        Ok(())
    }

    /// Perform cosine similarity vector search
    pub fn search(&self, collection: &str, query_vector: &[f32], limit: usize) -> Vec<ScoredPoint> {
        let Some(coll) = self.collections.get(collection) else {
            return Vec::new();
        };

        let mut scored: Vec<ScoredPoint> = coll
            .documents
            .values()
            .map(|doc| {
                let sim = Self::cosine_similarity(query_vector, &doc.vector);
                ScoredPoint {
                    id: doc.id.clone(),
                    score: sim,
                    payload: doc.payload.clone(),
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(limit);
        scored
    }

    /// Cosine similarity between two float vectors
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        for (&x, &y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        if norm_a <= 0.0 || norm_b <= 0.0 {
            0.0
        } else {
            (dot / (norm_a.sqrt() * norm_b.sqrt())).clamp(-1.0, 1.0)
        }
    }

    /// Purge all vectors tagged with a checkpoint newer than target_checkpoint
    pub fn purge_vectors_after(&mut self, target_checkpoint: &str) -> usize {
        let mut purged_count = 0;
        for coll in self.collections.values_mut() {
            coll.documents.retain(|_id, doc| {
                if let Some(cp_val) = doc.payload.get("checkpoint").and_then(|v| v.as_str()) {
                    if cp_val > target_checkpoint {
                        purged_count += 1;
                        return false;
                    }
                }
                true
            });
        }
        purged_count
    }

    /// Save index to disk
    pub fn save_to_disk(&self) -> Result<()> {
        if let Some(ref path) = self.storage_path {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let serialized = serde_json::to_vec(&self.collections)?;
            std::fs::write(path, serialized)?;
        }
        Ok(())
    }

    /// Load index from disk
    pub fn load_from_disk(&mut self) -> Result<()> {
        if let Some(ref path) = self.storage_path {
            if path.exists() {
                let data = std::fs::read(path)?;
                let colls: HashMap<String, VectorCollection> = serde_json::from_slice(&data)?;
                self.collections = colls;
            }
        }
        Ok(())
    }
}

pub type SharedVectorStore = Arc<RwLock<VectorStore>>;

pub fn create_shared_vector_store(storage_path: Option<PathBuf>) -> SharedVectorStore {
    let mut store = match storage_path {
        Some(p) => VectorStore::with_storage_path(p),
        None => VectorStore::new(),
    };
    let _ = store.load_from_disk();
    Arc::new(RwLock::new(store))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_vector_cosine_search() {
        let mut store = VectorStore::new();
        let mut payload = HashMap::new();
        payload.insert("doc".to_string(), serde_json::json!("STM32F401 SPI driver"));

        store.upsert("datasheets", "doc1", vec![1.0, 0.0, 0.0], payload.clone()).unwrap();
        store.upsert("datasheets", "doc2", vec![0.0, 1.0, 0.0], payload).unwrap();

        let results = store.search("datasheets", &[0.9, 0.1, 0.0], 1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "doc1");
        assert!(results[0].score > 0.8);
    }
}
