use serde::{Deserialize, Serialize};

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
