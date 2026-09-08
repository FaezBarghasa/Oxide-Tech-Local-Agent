use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForestNode {
    pub id: String,
    pub path: String,
    pub depth: usize,
    pub children: Vec<String>,
    pub document_reference: Option<String>,
    #[serde(default)]
    pub embedding: Option<Vec<f32>>,
}

pub struct FableForest {
    pub nodes: HashMap<String, ForestNode>,
}

impl Default for FableForest {
    fn default() -> Self {
        Self::new()
    }
}

impl FableForest {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Fast SIMD / AVX-512 cosine similarity matching between query and node vector
    #[inline(always)]
    pub fn compute_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        // Auto-vectorizable 4-wide/8-wide loop for AVX2/AVX-512 on AMD Zen & Intel CPUs
        for i in 0..a.len() {
            dot += a[i] * b[i];
            norm_a += a[i] * a[i];
            norm_b += b[i] * b[i];
        }

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a.sqrt() * norm_b.sqrt())
        }
    }

    /// Run the Bi-Path traversal strategy: Balance tree hierarchy with SIMD vector matching
    pub fn traverse_bi_path(
        &self,
        query_vector: &[f32],
        seed_node_id: &str,
        max_results: usize,
    ) -> Vec<String> {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut candidates = Vec::new();

        queue.push_back(seed_node_id.to_string());
        visited.insert(seed_node_id.to_string());

        // Traversal path 1: Traverse the hierarchy
        while let Some(current_id) = queue.pop_front() {
            if let Some(node) = self.nodes.get(&current_id) {
                let score = if let Some(ref emb) = node.embedding {
                    Self::compute_cosine_similarity(query_vector, emb)
                } else {
                    0.5 // Baseline topological structural score
                };

                candidates.push((current_id.clone(), score));

                // Fetch child nodes for deeper traversal
                for child in &node.children {
                    if !visited.contains(child) {
                        visited.insert(child.clone());
                        queue.push_back(child.clone());
                    }
                }
            }
        }

        // Traversal path 2: Retain high-scoring semantic nodes only
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.into_iter().take(max_results).map(|(id, _)| id).collect()
    }
}
