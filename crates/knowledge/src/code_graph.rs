use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::info;

/// Node types in the Multi-Modal Code Graph
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodeNodeType {
    Function,
    Struct,
    Trait,
    Enum,
    Module,
    Variable,
    Interface,
    /// External researched crate, library, or API node
    ExternalDependency,
    /// External documentation node
    ExternalDoc,
}

/// A node in the code graph representing an AST symbol or state construct
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeNode {
    pub id: String,
    pub name: String,
    pub node_type: CodeNodeType,
    pub file_path: String,
    pub span_start: usize,
    pub span_end: usize,
    pub signature: String,
    pub doc_comment: Option<String>,
    pub vector_id: Option<String>,
}

/// Relationship edge types in the Multi-Modal Code Graph
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodeEdgeType {
    /// Function calls another function
    Calls,
    /// Struct implements a trait
    Implements,
    /// Module defines a symbol
    Defines,
    /// File imports a module/symbol
    Imports,
    /// Function mutates/writes a variable/state
    Mutates,
    /// Function passes data to another function
    PassesTo,
    /// Symbol references another symbol
    References,
    /// Node depends on an external researched dependency/API
    DependsOnExternal,
}

/// A directed edge in the code graph
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeEdge {
    pub from_id: String,
    pub to_id: String,
    pub edge_type: CodeEdgeType,
    pub weight: f32,
}

/// Multi-Modal Code Graph: Topological mapping of AST, Call, Dependency, and Data Flow graphs
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MultiModalCodeGraph {
    pub nodes: HashMap<String, CodeNode>,
    pub outgoing_edges: HashMap<String, Vec<CodeEdge>>,
    pub incoming_edges: HashMap<String, Vec<CodeEdge>>,
}

impl MultiModalCodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            outgoing_edges: HashMap::new(),
            incoming_edges: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: CodeNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Add a directed edge between two nodes
    pub fn add_edge(&mut self, from_id: &str, to_id: &str, edge_type: CodeEdgeType, weight: f32) {
        let edge = CodeEdge {
            from_id: from_id.to_string(),
            to_id: to_id.to_string(),
            edge_type,
            weight,
        };

        self.outgoing_edges
            .entry(from_id.to_string())
            .or_default()
            .push(edge.clone());

        self.incoming_edges
            .entry(to_id.to_string())
            .or_default()
            .push(edge);
    }

    /// Retrieve all direct callers of a node (1-hop incoming calls edges)
    pub fn get_callers(&self, node_id: &str) -> Vec<&CodeNode> {
        if let Some(edges) = self.incoming_edges.get(node_id) {
            edges
                .iter()
                .filter(|e| e.edge_type == CodeEdgeType::Calls)
                .filter_map(|e| self.nodes.get(&e.from_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Retrieve all direct callees of a node (1-hop outgoing calls edges)
    pub fn get_callees(&self, node_id: &str) -> Vec<&CodeNode> {
        if let Some(edges) = self.outgoing_edges.get(node_id) {
            edges
                .iter()
                .filter(|e| e.edge_type == CodeEdgeType::Calls)
                .filter_map(|e| self.nodes.get(&e.to_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Traverse k-hop neighborhood using Breadth-First Search
    pub fn traverse_k_hop(&self, start_id: &str, k: usize) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut current_level = vec![start_id.to_string()];
        visited.insert(start_id.to_string());

        for _ in 0..k {
            let mut next_level = Vec::new();
            for node_id in current_level {
                if let Some(out_edges) = self.outgoing_edges.get(&node_id) {
                    for edge in out_edges {
                        if visited.insert(edge.to_id.clone()) {
                            next_level.push(edge.to_id.clone());
                        }
                    }
                }
                if let Some(in_edges) = self.incoming_edges.get(&node_id) {
                    for edge in in_edges {
                        if visited.insert(edge.from_id.clone()) {
                            next_level.push(edge.from_id.clone());
                        }
                    }
                }
            }
            if next_level.is_empty() {
                break;
            }
            current_level = next_level;
        }

        visited
    }

    /// Summary statistics of the graph topology
    pub fn stats(&self) -> (usize, usize) {
        let total_nodes = self.nodes.len();
        let total_edges: usize = self.outgoing_edges.values().map(|v| v.len()).sum();
        info!(
            "Code Graph topology: {} nodes, {} directed edges",
            total_nodes, total_edges
        );
        (total_nodes, total_edges)
    }
}
