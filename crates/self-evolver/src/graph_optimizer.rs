use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Graph Traversal Weight & Edge-Priority Optimizer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphWeightProfile {
    pub call_edge_weight: f32,
    pub data_flow_weight: f32,
    pub dependency_weight: f32,
    pub ast_scope_weight: f32,
    pub max_traversal_hops: usize,
}

impl Default for GraphWeightProfile {
    fn default() -> Self {
        Self {
            call_edge_weight: 1.0,
            data_flow_weight: 0.8,
            dependency_weight: 0.6,
            ast_scope_weight: 0.4,
            max_traversal_hops: 3,
        }
    }
}

pub struct GraphTraversalOptimizer {
    pub profile: GraphWeightProfile,
    pub feedback_history: HashMap<String, usize>, // missed dependency error counts by edge type
}

impl GraphTraversalOptimizer {
    pub fn new() -> Self {
        Self {
            profile: GraphWeightProfile::default(),
            feedback_history: HashMap::new(),
        }
    }

    /// Record a missed dependency event and auto-tune traversal weights
    pub fn record_missed_dependency(&mut self, missed_edge_type: &str) {
        info!("GraphOptimizer recording missed dependency on edge: {}", missed_edge_type);
        *self.feedback_history.entry(missed_edge_type.to_string()).or_insert(0) += 1;

        match missed_edge_type {
            "data_flow" | "mutates" => {
                self.profile.data_flow_weight = (self.profile.data_flow_weight + 0.15).min(2.0);
            }
            "calls" => {
                self.profile.call_edge_weight = (self.profile.call_edge_weight + 0.1).min(2.0);
            }
            "imports" | "dependency" => {
                self.profile.dependency_weight = (self.profile.dependency_weight + 0.2).min(2.0);
                self.profile.max_traversal_hops = (self.profile.max_traversal_hops + 1).min(5);
            }
            _ => {}
        }
    }

    /// Get current optimized traversal parameters
    pub fn get_current_profile(&self) -> GraphWeightProfile {
        self.profile.clone()
    }
}

impl Default for GraphTraversalOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
