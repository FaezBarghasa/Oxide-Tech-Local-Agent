#![allow(
    clippy::collapsible_if,
    clippy::new_without_default,
    clippy::should_implement_trait,
    clippy::too_many_arguments
)]

use serde::{Deserialize, Serialize};

pub mod agent_fsm;
pub mod agent_modes;
pub mod cloud_response_capture;
pub mod feedback;
pub mod hub_routing;
pub mod local_first_router;
pub mod lora_router;
pub mod observer;
pub mod srae;
pub mod supervisor;

pub use agent_fsm::{AgentFsmRouter, RoutingDecision};
pub use agent_modes::{
    AgentMode, PermissionDecision, RiskClass, ToolPermissions, classify_tool_call,
};
pub use lora_router::{DynamicLoraRouter, LoraAdapterConfig, LoraAdapterType};
pub use observer::{LoopObservationReport, LoopRecommendation, ObserverAgent};
pub use supervisor::{SubAgentRole, SupervisorAgent, TaskDag, TaskNode, TaskStatus};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub provider: String,
    pub cost_per_token_input: f64,
    pub cost_per_token_output: f64,
    pub latency_ms: u64,
    pub success_rate: f32,
    pub quality_score: f32,
}

pub struct ModelRouter;

impl ModelRouter {
    /// Select the model that scores the highest based on quality, latency, cost, and success rate.
    pub fn select_best_model(models: &[ModelInfo]) -> Option<ModelInfo> {
        let mut best_model = None;
        let mut best_score = -1.0;

        for m in models {
            // score = quality + latency_score + cost_score + success_rate

            // Normalize latency (cap at 10 seconds / 10,000ms)
            let latency_score = 1.0 - (m.latency_ms as f64 / 10000.0).min(1.0);

            // Normalize cost (assume $30 per million tokens input + output is maximum acceptable)
            let total_cost_per_million =
                (m.cost_per_token_input + m.cost_per_token_output) * 1_000_000.0;
            let cost_score = 1.0 - (total_cost_per_million / 30.0).min(1.0);

            let score = m.quality_score as f64 + latency_score + cost_score + m.success_rate as f64;

            if score > best_score {
                best_score = score;
                best_model = Some(m.clone());
            }
        }

        best_model
    }
}
