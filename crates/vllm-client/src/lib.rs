pub mod candle_engine;
pub mod client;
pub mod lora_hot_swap;
pub mod lora_router;
pub mod thinker;

pub use candle_engine::{CandleEngine, CandleEngineConfig, ComputeDevice};
pub use client::{LlmProvider, LlmRouterClient};
pub use lora_hot_swap::LoraHotSwapManager;
pub use lora_router::DynamicLoraRouter;
pub use thinker::{ThinkerClient, ThinkerOutput};

use anyhow::Result;

/// Common interface for any local or remote LLM inference engine.
#[async_trait::async_trait]
pub trait InferenceEngine: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String>;
    async fn complete_json(&self, prompt: &str) -> Result<serde_json::Value>;
}
