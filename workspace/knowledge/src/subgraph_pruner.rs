use std::collections::HashSet;
use crate::code_graph::{CodeEdge, CodeNode, MultiModalCodeGraph};

/// A pruned, self-contained sub-graph suitable for LLM prompt context injection
#[derive(Debug, Clone)]
pub struct PrunedSubgraph {
    pub center_node: Option<CodeNode>,
    pub neighbor_nodes: Vec<CodeNode>,
    pub internal_edges: Vec<CodeEdge>,
    pub estimated_tokens: usize,
}

impl PrunedSubgraph {
    /// Format the pruned subgraph into clean, token-efficient Markdown context for the LLM
    pub fn to_markdown_context(&self) -> String {
        let mut out = String::from("### Topology-Aware Subgraph Context\n\n");

        if let Some(ref center) = self.center_node {
            out.push_str(&format!(
                "**Primary Target Symbol**: `{}` ({:?})\n- File: `{}:{}-{}`\n- Signature: `{}`\n\n",
                center.name, center.node_type, center.file_path, center.span_start, center.span_end, center.signature
            ));
        }

        if !self.neighbor_nodes.is_empty() {
            out.push_str("#### Connected Neighbors (1-Hop Topology):\n");
            for node in &self.neighbor_nodes {
                out.push_str(&format!(
                    "- `{}` ({:?}) in `{}` => `{}`\n",
                    node.name, node.node_type, node.file_path, node.signature
                ));
            }
            out.push('\n');
        }

        if !self.internal_edges.is_empty() {
            out.push_str("#### Relational Dependency Paths:\n");
            for edge in &self.internal_edges {
                out.push_str(&format!(
                    "- `{}` --[{:?}]--> `{}`\n",
                    edge.from_id, edge.edge_type, edge.to_id
                ));
            }
            out.push('\n');
        }

        out
    }
}

pub struct SubgraphPruner;

impl SubgraphPruner {
    /// Extract a minimal induced k-hop subgraph around target_id
    pub fn prune(graph: &MultiModalCodeGraph, target_id: &str, k_hops: usize) -> PrunedSubgraph {
        let center_node = graph.nodes.get(target_id).cloned();
        let neighbor_ids = graph.traverse_k_hop(target_id, k_hops);

        let mut neighbor_nodes = Vec::new();
        for id in &neighbor_ids {
            if id != target_id {
                if let Some(node) = graph.nodes.get(id) {
                    neighbor_nodes.push(node.clone());
                }
            }
        }

        // Collect internal edges between all nodes in the neighborhood
        let mut internal_edges = Vec::new();
        let all_ids: HashSet<&String> = neighbor_ids.iter().collect();

        for from_id in &neighbor_ids {
            if let Some(out_edges) = graph.outgoing_edges.get(from_id) {
                for edge in out_edges {
                    if all_ids.contains(&edge.to_id) {
                        internal_edges.push(edge.clone());
                    }
                }
            }
        }

        // Approximate token count (character count / 4)
        let total_chars = center_node.as_ref().map(|c| c.signature.len()).unwrap_or(0)
            + neighbor_nodes.iter().map(|n| n.signature.len() + n.name.len()).sum::<usize>()
            + internal_edges.len() * 30;

        let estimated_tokens = (total_chars / 4).max(1);

        PrunedSubgraph {
            center_node,
            neighbor_nodes,
            internal_edges,
            estimated_tokens,
        }
    }
}
