use anyhow::{Result, anyhow};
use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An atomic action performed by the agent or sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionNode {
    pub id: String,
    pub action_type: String, // e.g. "tool_call", "thought", "code_exec", "verification"
    pub description: String,
    pub payload: serde_json::Value,
    pub success: bool,
    pub timestamp: i64,
}

/// Relationship between causal actions in the trajectory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionEdgeType {
    CausedBy,
    CorrectedBy,
    FollowsFrom,
}

/// Direct acyclic graph representing causal action steps and decision paths.
pub struct ActionGraph {
    graph: DiGraph<ActionNode, ActionEdgeType>,
    index_map: HashMap<String, NodeIndex>,
}

impl Default for ActionGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index_map: HashMap::new(),
        }
    }

    /// Add an action node to the graph.
    pub fn add_action(&mut self, action: ActionNode) -> NodeIndex {
        let id = action.id.clone();
        let idx = self.graph.add_node(action);
        self.index_map.insert(id, idx);
        idx
    }

    /// Link two actions causally.
    pub fn add_causal_link(
        &mut self,
        from_id: &str,
        to_id: &str,
        edge_type: ActionEdgeType,
    ) -> Result<()> {
        let from_idx = self
            .index_map
            .get(from_id)
            .copied()
            .ok_or_else(|| anyhow!("Source action node not found: {}", from_id))?;
        let to_idx = self
            .index_map
            .get(to_id)
            .copied()
            .ok_or_else(|| anyhow!("Target action node not found: {}", to_id))?;

        self.graph.add_edge(from_idx, to_idx, edge_type);
        Ok(())
    }

    /// Get all causal predecessors of an action node.
    pub fn get_predecessors(&self, action_id: &str) -> Vec<&ActionNode> {
        let mut result = Vec::new();
        if let Some(&idx) = self.index_map.get(action_id) {
            for edge in self.graph.edges_directed(idx, Direction::Incoming) {
                let source_idx = edge.source();
                if let Some(node) = self.graph.node_weight(source_idx) {
                    result.push(node);
                }
            }
        }
        result
    }

    /// Return total nodes in the action graph.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Find all actions that failed and their subsequent corrections.
    pub fn get_failure_correction_pairs(&self) -> Vec<(&ActionNode, &ActionNode)> {
        let mut pairs = Vec::new();
        for edge in self.graph.edge_references() {
            if matches!(edge.weight(), ActionEdgeType::CorrectedBy) {
                if let (Some(failed), Some(correction)) = (
                    self.graph.node_weight(edge.source()),
                    self.graph.node_weight(edge.target()),
                ) {
                    pairs.push((failed, correction));
                }
            }
        }
        pairs
    }
}
