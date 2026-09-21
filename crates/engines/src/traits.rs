use async_trait::async_trait;
use llama_cpp_2::Llama;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::Error;

pub mod generation_params;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct GenerationParams {
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: usize,
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub params: GenerationParams,
}

#[derive(Debug)]
pub enum EngineError {
    ModelNotLoaded,
    GenerationFailed(String),
    UnloadFailed(String),
    InvalidPrompt,
    TokenLimitExceeded,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::ModelNotLoaded => write!(f, "Model not loaded"),
            EngineError::GenerationFailed(msg) => write!(f, "Generation failed: {}", msg),
            EngineError::UnloadFailed(msg) => write!(f, "Unload failed: {}", msg),
            EngineError::InvalidPrompt => write!(f, "Invalid prompt"),
            EngineError::TokenLimitExceeded => write!(f, "Token limit exceeded"),
        }
    }
}

impl std::error::Error for EngineError {}

#[async_trait]
pub trait InferenceProvider: Send + Sync + 'static {
    /// Returns the engine type identifier (e.g., "llama.cpp", "mistralrs", "sidecar-vllm")
    fn engine_name(&self) -> &str;

    /// Streams generated tokens back via the provided channel.
    /// The caller should send completion markers etc. through this channel.
    async fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), EngineError>;

    /// Unloads the model from VRAM/RAM, freeing resources.
    async fn unload(&self) -> Result<(), EngineError>;

    /// Returns provider-specific capabilities
    fn capabilities(&self) -> Capabilities {
        Capabilities::default()
    }

    /// Provider display name for logging and telemetry.
    fn provider_name(&self) -> &str {
        "unknown"
    }
}

#[derive(Debug, Clone, Default)]
pub struct Capabilities {
    pub supports_streaming: bool,
    pub supports_logprobs: bool,
    pub supports_stop_tokens: bool,
    pub max_context_length: usize,
}

#[derive(Debug, Clone)]
pub struct InferenceHealth {
    pub healthy: bool,
    pub active_model: Option<String>,
    pub vram_used_mb: Option<usize>,
    pub system_memory_used_mb: Option<usize>,
    pub queue_depth: usize,
}