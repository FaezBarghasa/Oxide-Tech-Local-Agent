use serde::{Deserialize, Serialize};
use surrealdb_types::RecordId;
use surrealdb_types::SurrealValue;

pub use tree_sitter_service::ast::{ParsedSymbol, SymbolField, SymbolMethod};

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct SymbolRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
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

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub name: String,
    pub mcu_type: String,
    pub target_triple: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct Component {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub ref_des: String,
    pub value: String,
    pub footprint: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct CompilationRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub project_id: RecordId,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct CloudTrainingSampleRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub prompt: String,
    pub context: common::contracts::ContextSnapshot,
    pub cloud_response: common::contracts::CloudResponse,
    pub validation: common::contracts::ValidationResult,
    pub outcome: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

