use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Enum)]
pub enum TaskType {
    Architecture,
    Debugging,
    Syntax,
    PcbLayout,
    SceneModeling,
    CodeCompletion,
    Training,
}

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Enum)]
pub enum UserOutcome {
    Accepted,
    Modified,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct ContextSnapshot {
    pub files: HashMap<String, String>,
    pub ast_summary: Option<String>,
    pub board_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct ValidationResult {
    pub cargo_check: bool,
    pub clippy: bool,
    pub geiger: bool,
    pub qemu: bool,
    pub hardware: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct CloudResponse {
    pub provider: String,
    pub model: String,
    pub content: String,
    pub confidence: f32,
    pub tokens: u32,
    pub cost: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct CloudTrainingSample {
    pub id: Uuid,
    pub tenant_id: String,
    pub user_id: String,
    pub task_type: String,
    pub prompt: String,
    pub cloud_provider: String,
    pub cloud_response: String,
    pub cloud_reasoning: Option<String>,
    pub validation: ValidationResult,
    pub confidence: f32,
    pub cost_usd: f32,
    pub user_outcome: UserOutcome,
}

#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct InferenceRequest {
    pub task_type: TaskType,
    pub prompt: String,
    pub context: ContextSnapshot,
    pub tenant: String,
    pub local_failures: u32,
}
