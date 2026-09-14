use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderKind {
    Ollama,
    Sglang,
    LlamaCpp,
    Candle,
    OpenAiCompatible,
    SafetensorsDirect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceCapabilities {
    pub provider: ProviderKind,
    pub supports_streaming: bool,
    pub supports_tools: bool,
    pub supports_lora_hotswap: bool,
    pub supports_json_mode: bool,
    pub supports_vision: bool,
    pub max_context_tokens: usize,
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub delta: String,
    pub is_final: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendHealth {
    pub healthy: bool,
    pub provider_name: String,
    pub active_model: String,
    pub memory_used_mb: Option<usize>,
}

pub type StreamResult = Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>;

/// Pluggable Inference Provider abstraction across Ollama, SGLang, llama.cpp, Candle, and vLLM
#[async_trait]
pub trait InferenceProvider: Send + Sync {
    fn capabilities(&self) -> InferenceCapabilities;
    async fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse>;
    async fn stream_chat(&self, req: ChatRequest) -> Result<StreamResult>;
    async fn health(&self) -> Result<BackendHealth>;
    async fn activate_lora(&self, _adapter_id: &str) -> Result<()> {
        Ok(())
    }
}
