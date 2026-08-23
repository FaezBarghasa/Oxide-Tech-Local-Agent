use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubgraphSlice {
    pub name: String,
    pub node_type: String,
    pub file_path: String,
    pub span: (usize, usize),
    pub calls: Vec<String>,
    pub uses_types: Vec<String>,
}

pub struct GraphContextSlicer;

impl GraphContextSlicer {
    pub fn format_pruned_subgraph_context(slice: &SubgraphSlice) -> String {
        format!(
            "```json\n{}\n```",
            serde_json::to_string_pretty(slice).unwrap_or_else(|_| "{}".to_string())
        )
    }
}
