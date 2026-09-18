use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub capabilities: Vec<String>,
    pub required_tools: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub checksum_blake3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterManifest {
    pub adapter_id: Uuid,
    pub base_model: String,
    pub target_domain: String,
    pub lora_rank: u32,
    pub lora_alpha: u32,
    pub file_path: String,
    pub checksum_blake3: String,
    pub minisign_signature: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryScope {
    Global,
    Session,
    Task,
    Scratchpad,
}
