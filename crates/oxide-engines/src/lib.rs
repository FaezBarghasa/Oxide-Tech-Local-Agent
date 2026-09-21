use async_trait::async_trait;
use oxide_core::{ChatMessage, GenerationParams, OxideError};
use tokio::sync::mpsc;

#[async_trait]
pub trait InferenceProvider: Send + Sync + 'static {
    /// Returns the engine identifier (e.g., "candle", "llama.cpp", "mistral.rs", "vllm-sidecar")
    fn engine_name(&self) -> &str;

    /// Streams token strings back via mpsc sender
    async fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError>;

    /// Unloads model weights from memory
    async fn unload(&self) -> Result<(), OxideError>;
}

pub mod sidecar;
pub mod mock;
pub mod candle_provider;
pub mod llama_cpp;

pub use sidecar::SidecarProvider;
pub use mock::MockProvider;
pub use candle_provider::CandleProvider;
pub use llama_cpp::LlamaCppProvider;