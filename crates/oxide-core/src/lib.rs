use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageUrlInfo {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrlInfo },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

impl fmt::Display for MessageContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageContent::Text(t) => write!(f, "{}", t),
            MessageContent::Parts(parts) => {
                for p in parts {
                    match p {
                        ContentPart::Text { text } => write!(f, "{}", text)?,
                        ContentPart::ImageUrl { image_url } => {
                            write!(f, "[Image: {}]", image_url.url)?
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

impl From<String> for MessageContent {
    fn from(s: String) -> Self {
        MessageContent::Text(s)
    }
}

impl From<&str> for MessageContent {
    fn from(s: &str) -> Self {
        MessageContent::Text(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ChatMessage {
    pub fn new_text(role: Role, text: impl Into<String>) -> Self {
        Self {
            role,
            content: MessageContent::Text(text.into()),
            name: None,
        }
    }

    pub fn text_content(&self) -> String {
        self.content.to_string()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParams {
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default)]
    pub top_k: Option<u32>,
    #[serde(default)]
    pub min_p: Option<f32>,
    #[serde(default)]
    pub presence_penalty: Option<f32>,
    #[serde(default)]
    pub repetition_penalty: Option<f32>,
    pub max_tokens: Option<usize>,
    pub stop: Option<Vec<String>>,
    #[serde(default)]
    pub stream: bool,
}

fn default_temperature() -> f32 {
    0.7
}

fn default_top_p() -> f32 {
    0.95
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            temperature: default_temperature(),
            top_p: default_top_p(),
            top_k: None,
            min_p: None,
            presence_penalty: None,
            repetition_penalty: None,
            max_tokens: Some(4096),
            stop: None,
            stream: false,
        }
    }
}

impl GenerationParams {
    /// Optimal sampling parameters for Ternary-Bonsai reasoning mode.
    pub fn bonsai_thinking_mode() -> Self {
        Self {
            temperature: 1.0,
            top_p: 0.95,
            top_k: Some(20),
            min_p: Some(0.0),
            presence_penalty: Some(0.0),
            repetition_penalty: Some(1.0),
            max_tokens: Some(32768),
            stop: None,
            stream: true,
        }
    }

    /// Optimal sampling parameters for Ornith-1.5 agentic coding and tool-calling precision.
    pub fn ornith_agent_mode() -> Self {
        Self {
            temperature: 0.2,
            top_p: 0.95,
            top_k: Some(40),
            min_p: Some(0.05),
            presence_penalty: Some(0.0),
            repetition_penalty: Some(1.05),
            max_tokens: Some(16384),
            stop: Some(vec![
                "<|im_end|>".to_string(),
                "<|endoftext|>".to_string(),
            ]),
            stream: true,
        }
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationChunk {
    pub token: String,
    pub is_final: bool,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMetrics {
    pub timestamp: i64,
    pub cpu_usage_pct: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub gpu_name: Option<String>,
    pub gpu_util_pct: Option<f32>,
    pub gpu_vram_used_mb: Option<u64>,
    pub gpu_vram_total_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub id: String,
    pub name: String,
    pub backend: String,
    pub context_length: usize,
    pub vram_usage_mb: Option<u64>,
}

#[derive(Error, Debug)]
pub enum OxideError {
    #[error("Inference engine error: {0}")]
    Engine(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Kernel error: {0}")]
    Kernel(String),
    #[error("Runtime error: {0}")]
    Runtime(String),
    #[error("Security violation: {0}")]
    SecurityViolation(String),
    #[error("FFI boundary error: {0}")]
    FFI(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub mod channel;
pub mod state_machine;
pub mod topology;
pub mod ffi_boundary;
pub mod dynamic_loader;

pub use channel::{TokenReceiver, TokenSender, create_token_channel};
pub use state_machine::{AgentState, AgentStateMachine, StateTransition};
pub use topology::RuntimeTopology;
pub use ffi_boundary::call_ffi_safe;
pub use dynamic_loader::{DynamicSkillLoader, SkillFn};
