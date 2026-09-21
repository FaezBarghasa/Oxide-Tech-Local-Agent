use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub struct OxideMcpServer {
    pub tools: Vec<McpToolDefinition>,
}

impl Default for OxideMcpServer {
    fn default() -> Self {
        Self {
            tools: vec![
                McpToolDefinition {
                    name: "oxide_stair_search".to_string(),
                    description: "Hierarchical STAIR Code-ToC leaf symbol search".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": { "type": "string" }
                        },
                        "required": ["query"]
                    }),
                },
                McpToolDefinition {
                    name: "oxide_remember".to_string(),
                    description: "Store persistent architectural decision into SurrealDB memory"
                        .to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "decision": { "type": "string" },
                            "kind": { "type": "string" }
                        },
                        "required": ["decision"]
                    }),
                },
            ],
        }
    }
}

impl OxideMcpServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list_tools(&self) -> &[McpToolDefinition] {
        &self.tools
    }
}
