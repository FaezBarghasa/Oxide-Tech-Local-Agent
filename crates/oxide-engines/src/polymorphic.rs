use async_trait::async_trait;
use std::path::Path;
use tokio::sync::mpsc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Model not loaded: {0}")]
    ModelNotLoaded(String),
    #[error("Inference generation failed: {0}")]
    GenerationFailed(String),
    #[error("LoRA adapter error: {0}")]
    LoraError(String),
    #[error("Hardware execution error: {0}")]
    HardwareError(String),
    #[error("Alignment fault at offset {offset}: misaligned by {misalignment} for {alignment}")]
    AlignmentFault {
        offset: usize,
        alignment: usize,
        misalignment: usize,
    },
    #[error("Out of bounds: requested {requested}, available {available}")]
    OutOfBounds {
        requested: usize,
        available: usize,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InferenceRequest {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub stop_sequences: Vec<String>,
    pub lora_adapter: Option<String>,
}

impl Default for InferenceRequest {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            max_tokens: 512,
            temperature: 0.7,
            top_p: 0.9,
            stop_sequences: Vec::new(),
            lora_adapter: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InferenceChunk {
    Token(String),
    Usage {
        prompt_tokens: usize,
        completion_tokens: usize,
    },
    Done,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EngineCapabilities {
    pub supports_streaming: bool,
    pub supports_lora: bool,
    pub supports_speculative: bool,
    pub max_context_tokens: usize,
    pub supported_quantizations: Vec<String>,
}

#[async_trait]
pub trait PolymorphicInferenceProvider: Send + Sync {
    async fn infer_stream(
        &self,
        request: InferenceRequest,
        tx: mpsc::Sender<Result<InferenceChunk, EngineError>>,
    ) -> Result<(), EngineError>;

    fn capabilities(&self) -> EngineCapabilities;
    async fn load_lora(&self, adapter_path: &Path) -> Result<(), EngineError>;
    async fn unload_lora(&self, adapter_name: &str) -> Result<(), EngineError>;
}
