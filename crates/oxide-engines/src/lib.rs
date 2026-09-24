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

pub mod candle_provider;
pub mod llama_cpp;
pub mod mistral_rs;
pub mod mmap_tensor;
pub mod mock;
pub mod prism_sidecar;
pub mod sidecar;

pub use candle_provider::CandleProvider;
pub use llama_cpp::LlamaCppProvider;
pub use mistral_rs::MistralRsProvider;
pub use mmap_tensor::{
    AlignedTensorMap, GgufTensorInfo, GgufTensorType, MemoryAdvice, MmapModel, TensorSlice,
};
pub use mock::MockProvider;
pub use prism_sidecar::PrismBonsaiEngine;
pub use sidecar::SidecarProvider;
