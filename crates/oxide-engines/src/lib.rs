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
pub mod decision_engine;
pub mod grammar;
pub mod llama_cpp;
pub mod mistral_rs;
pub mod mmap_tensor;
pub mod mobile;
pub mod polymorphic;
pub mod prism_sidecar;
pub mod sidecar;
pub mod tiered_kv_cache;
pub mod universal_loader;

pub use decision_engine::{
    CandidateVectorCache, DecisionDevice, DecisionEngine, DecisionInput, DecisionOutput,
    FastKanDecisionHead, SpeculativeCascadeConfig,
};
pub use grammar::{GbnfCompiler, GrammarRule};
pub use mobile::MobileDecisionEngine;
pub use polymorphic::{
    EngineCapabilities, EngineError, InferenceChunk, InferenceRequest, PolymorphicInferenceProvider,
};
pub use tiered_kv_cache::{
    KvCacheConfig, KvMemoryTier, KvPage, KvTierMetrics, TieredKvCacheManager,
};
pub use universal_loader::{ContainerFormat, QuantType, TensorDescriptor, UniversalModelContainer};

pub use candle_provider::CandleProvider;
pub use llama_cpp::LlamaCppProvider;
pub use mistral_rs::MistralRsProvider;
pub use mmap_tensor::{
    AlignedTensorMap, GgufTensorInfo, GgufTensorType, HardenedTensorMap, MemoryAdvice, MmapModel,
    TensorSlice,
};
pub use prism_sidecar::PrismBonsaiEngine;
pub use sidecar::SidecarProvider;

