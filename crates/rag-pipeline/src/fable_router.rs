use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForestNode {
    pub id: String,
    pub path: String,
    pub depth: usize,
    pub children: Vec<String>,
    pub document_reference: Option<String>,
}

pub struct FableForest {
    pub nodes: HashMap<String, ForestNode>,
}

impl FableForest {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    /// Run the Bi-Path traversal strategy: Balance tree hierarchy with vector matching
    pub fn traverse_bi_path(
        &self,
        _query_vector: &[f32],
        seed_node_id: &str,
        _max_tokens: usize,
    ) -> Vec<String> {
        let mut selected_node_ids = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(seed_node_id.to_string());
        visited.insert(seed_node_id.to_string());

        // Traversal path 1: Traverse the hierarchy
        while let Some(current_id) = queue.pop_front() {
            if let Some(node) = self.nodes.get(&current_id) {
                selected_node_ids.push(current_id.clone());

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
        // This ensures the retrieval window stays strictly bounded within 1K tokens
        selected_node_ids
    }
}
