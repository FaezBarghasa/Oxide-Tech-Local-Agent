use serde::{Deserialize, Serialize};
use surrealdb_types::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub project_id: RecordId,
    pub name: String,
    pub status: String, // "pending" | "running" | "success" | "failed"
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub name: String,
    pub role: String, // "thinker" | "planner" | "coder" | "verifier"
    pub system_prompt: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub provider: String,
    pub name: String,
    pub cost_input: f64,
    pub cost_output: f64,
    pub latency: u64, // in ms
    pub success_rate: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub task_id: RecordId,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub task: String,
    pub success: bool,
    pub iterations: u32,
    pub intent: String,
    pub pattern: String,
    pub architecture: String,
    pub verification_result: String,
    pub outcome: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub title: String,
    pub source: String, // "docs.rs" | "local" | "crates.io"
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub name: String,
    pub description: String,
    pub domain: String, // "Embedded" | "Backend" | "PCB" | "CAD" | "Simulation" | "Documentation"
    pub skill_type: String, // "Generate" | "Refactor" | "Review" | "Design" | "Upgrade" | "Analyze"
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub task_id: RecordId,
    pub tool_name: String,
    pub input: String,
    pub output: String,
    pub success: bool,
    pub duration_ms: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub task_id: RecordId,
    pub verifier_name: String,
    pub pass: bool,
    pub score: f32,
    pub output: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
