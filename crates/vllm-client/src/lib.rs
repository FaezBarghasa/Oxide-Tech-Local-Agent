pub mod agentic_loop;
pub mod candle_engine;
pub mod client;
pub mod llama_server;
pub mod lora_hot_swap;
pub mod lora_router;
pub mod multi_agent;
pub mod ollama;
pub mod provider;
pub mod safetensors_loader;
pub mod sglang_provider;
pub mod speculative_engine;
pub mod thinker;
pub mod tool_call_parser;

pub use agentic_loop::{AgenticLoopRunner, SimpleToolExecutor, ToolExecutor};
pub use candle_engine::{CandleEngine, CandleEngineConfig, ComputeDevice};
pub use client::{LlmProvider, LlmRouterClient};
pub use llama_server::LlamaServerProvider;
pub use lora_hot_swap::LoraHotSwapManager;
pub use lora_router::DynamicLoraRouter;
pub use multi_agent::{
    AgentBuilder, AgentMessage, AgentRecord, AgentRole, AgentThread, MultiAgentCoordinator,
    PeerDialogueResult,
};
pub use ollama::OllamaProvider;
pub use provider::{
    BackendHealth, ChatMessage, ChatRequest, ChatResponse, ConversationTurn, InferenceCapabilities,
    InferenceProvider, ProviderKind, StreamChunk, StreamResult, ToolCall, ToolDefinition,
};
pub use safetensors_loader::{SafetensorModelLoader, SafetensorModelSummary, TensorMeta};
pub use sglang_provider::SglangProvider;
pub use speculative_engine::SpeculativeDecodingEngine;
pub use thinker::{ThinkerClient, ThinkerOutput};
pub use tool_call_parser::ToolCallParser;

use anyhow::Result;

/// Common interface for any local or remote LLM inference engine.
#[async_trait::async_trait]
pub trait InferenceEngine: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String>;
    async fn complete_json(&self, prompt: &str) -> Result<serde_json::Value>;
}
