use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

// ── Provider Kind ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderKind {
    Ollama,
    Sglang,
    LlamaCpp,
    LlamaServer,
    Candle,
    OpenAiCompatible,
    SafetensorsDirect,
}

// ── Capability Flags ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceCapabilities {
    pub provider: ProviderKind,
    pub supports_streaming: bool,
    pub supports_tool_calls: bool,
    pub supports_lora_hotswap: bool,
    pub supports_json_mode: bool,
    pub supports_grammar_constrained: bool,
    pub supports_speculative_decoding: bool,
    pub supports_prefix_cache: bool,
    pub supports_multimodal: bool,
    pub context_window: usize,
}

// ── Tool Definitions (MCP schema-compatible) ──────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub r#type: String,
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameters {
    pub r#type: String, // always "object"
    pub properties: std::collections::HashMap<String, ToolParameter>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
}

// ── Tool Call (model output) ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

// ── Conversation Turn (multi-turn agentic history) ────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    /// "system" | "user" | "assistant" | "tool"
    pub role: String,
    pub content: String,
    /// Set when role == "tool" to link back to a ToolCall.id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool name (when role == "tool")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool calls emitted by the assistant (when role == "assistant")
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tool_calls: Vec<ToolCall>,
}

impl ConversationTurn {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
            tool_call_id: None,
            name: None,
            tool_calls: vec![],
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
            tool_call_id: None,
            name: None,
            tool_calls: vec![],
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
            tool_call_id: None,
            name: None,
            tool_calls: vec![],
        }
    }

    pub fn tool_result(tool_call_id: impl Into<String>, name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: "tool".into(),
            content: content.into(),
            tool_call_id: Some(tool_call_id.into()),
            name: Some(name.into()),
            tool_calls: vec![],
        }
    }
}

// ── Chat Request / Response ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub json_mode: Option<bool>,
    /// Full conversation history for multi-turn agentic loops
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub history: Vec<ConversationTurn>,
    /// MCP-compatible tool definitions provided to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    /// "auto" | "required" | "none" | specific tool name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    /// Base64-encoded images for multimodal models (Llava, Moondream, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// BNF/GBNF grammar for grammar-constrained output (llama-server / SGLang)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grammar: Option<String>,
    /// Stop sequences (early termination tokens)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// llama-server slot ID for KV prefix cache stickiness
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub finish_reason: Option<String>,
    /// Structured tool calls extracted from the model response
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
    /// Wall-clock inference latency in milliseconds
    pub latency_ms: u64,
    /// llama-server slot ID for follow-up requests (prefix cache reuse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<i32>,
}

// ── Streaming ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub delta: String,
    pub is_final: bool,
    #[serde(default)]
    pub tool_calls_delta: Vec<ToolCall>,
}

pub type StreamResult = Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>;

// ── Backend Health ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendHealth {
    pub healthy: bool,
    pub provider_name: String,
    pub active_model: String,
    pub memory_used_mb: Option<usize>,
    pub vram_used_mb: Option<usize>,
    pub available_slots: Option<usize>,
    pub queue_depth: Option<usize>,
}

// ── InferenceProvider Trait ───────────────────────────────────────────────────

/// Pluggable inference provider abstraction across Ollama, SGLang,
/// llama-server, llama.cpp, Candle, vLLM and all OpenAI-compatible endpoints.
#[async_trait]
pub trait InferenceProvider: Send + Sync {
    fn capabilities(&self) -> InferenceCapabilities;

    async fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse>;

    async fn stream_chat(&self, req: ChatRequest) -> Result<StreamResult>;

    async fn health(&self) -> Result<BackendHealth>;

    /// Activate a LoRA adapter by name/path. No-op if not supported.
    async fn activate_lora(&self, _adapter_id: &str) -> Result<()> {
        Ok(())
    }

    /// Deactivate / unload the active LoRA adapter.
    async fn deactivate_lora(&self) -> Result<()> {
        Ok(())
    }

    /// Provider display name for logging and telemetry.
    fn provider_name(&self) -> &str {
        "unknown"
    }
}
