use tree_sitter::{Parser, Node, Tree};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolField {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolMethod {
    pub name: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedSymbol {
    pub name: String,
    pub kind: String, // "struct" | "enum" | "trait" | "impl" | "function"
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub doc_comment: Option<String>,
    pub fields: Option<Vec<SymbolField>>,
    pub variants: Option<Vec<String>>,
    pub methods: Option<Vec<SymbolMethod>>,
    pub implements_trait: Option<String>,
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CodeNodeType {
    File { path: String },
    Module { name: String },
    Struct { name: String, is_no_std: bool },
    Trait { name: String },
    Function { name: String, is_async: bool, is_unsafe: bool, return_type: Option<String> },
    Field { name: String, type_name: String },
    Variable { name: String, type_name: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphNode {
    pub id: String,
    pub node_type: CodeNodeType,
    pub span: (usize, usize), // (start_byte, end_byte)
    pub file_path: String,
    pub doc_comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CodeEdgeType {
    Defines,      // Module -> Struct / Function
    Calls,        // Function -> Function
    Implements,   // Struct -> Trait
    References,   // Function -> Struct / Field
    DataFlowsTo,  // Variable -> Variable
    Imports,      // File -> Module
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphEdge {
    pub from: String,
    pub to: String,
    pub edge_type: CodeEdgeType,
}

pub struct AstGraphExtractor {
    rust_parser: Parser,
}

impl AstGraphExtractor {
    pub fn new() -> Self {
        let mut rust_parser = Parser::new();
        rust_parser.set_language(tree_sitter_rust::language()).expect("Failed loading Rust grammar");
        Self { rust_parser }
    }

    pub fn parse_rust_file(&mut self, file_path: &str, source: &str) -> (Vec<CodeGraphNode>, Vec<CodeGraphEdge>) {
        let tree: Tree = self.rust_parser.parse(source, None).unwrap();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let file_node_id = format!("file:{}", file_path);
        nodes.push(CodeGraphNode {
            id: file_node_id.clone(),
            node_type: CodeNodeType::File { path: file_path.to_string() },
            span: (0, source.len()),
            file_path: file_path.to_string(),
            doc_comment: None,
        });

        self.traverse_tree(tree.root_node(), source, file_path, &file_node_id, &mut nodes, &mut edges);
        (nodes, edges)
    }

    fn traverse_tree(
        &self,
        node: Node,
        source: &str,
        file_path: &str,
        parent_id: &str,
        nodes: &mut Vec<CodeGraphNode>,
        edges: &mut Vec<CodeGraphEdge>,
    ) {
        let mut current_id = parent_id.to_string();

        match node.kind() {
            "function_item" => {
                let name = node.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "anonymous".to_string());
                
                let fn_source = &source[node.start_byte()..node.end_byte()];
                let prefix = fn_source.split("fn ").next().unwrap_or("");
                let is_async = prefix.contains("async");
                let is_unsafe = prefix.contains("unsafe");
                let func_id = format!("fn:{}:{}", file_path, name);

                nodes.push(CodeGraphNode {
                    id: func_id.clone(),
                    node_type: CodeNodeType::Function {
                        name,
                        is_async,
                        is_unsafe,
                        return_type: node.child_by_field_name("return_type").and_then(|n| n.utf8_text(source.as_bytes()).ok()).map(|s| s.to_string()),
                    },
                    span: (node.start_byte(), node.end_byte()),
                    file_path: file_path.to_string(),
                    doc_comment: None,
                });

                edges.push(CodeGraphEdge {
                    from: parent_id.to_string(),
                    to: func_id.clone(),
                    edge_type: CodeEdgeType::Defines,
                });
                current_id = func_id;
            }
            "call_expression" => {
                if let Some(function) = node.child_by_field_name("function") {
                    let callee_name = function.utf8_text(source.as_bytes()).unwrap_or_default();
                    edges.push(CodeGraphEdge {
                        from: parent_id.to_string(),
                        to: format!("callee:{}", callee_name),
                        edge_type: CodeEdgeType::Calls,
                    });
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_tree(child, source, file_path, &current_id, nodes, edges);
        }
    }
}

impl Default for AstGraphExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_ast_graph_extractor_parsing() {
        let mut extractor = AstGraphExtractor::new();
        let code = r#"
            pub async fn fetch_telemetry() -> Result<TelemetryData, Error> {
                let conn = connect_db();
                conn.query()
            }
        "#;

        let (nodes, edges) = extractor.parse_rust_file("src/telemetry.rs", code);
        assert!(!nodes.is_empty());
        assert!(!edges.is_empty());

        let has_fn = nodes.iter().any(|n| match &n.node_type {
            CodeNodeType::Function { name, is_async, .. } => name == "fetch_telemetry" && *is_async,
            _ => false,
        });
        assert!(has_fn);
    }
}
