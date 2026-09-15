use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Step-level invariant execution metrics for granular agent evaluation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StepInvariantMetrics {
    /// Ratio of valid tool selections without schema hallucination (0.0 to 1.0)
    pub tool_selection_accuracy: f64,
    /// Ratio of tool calls conforming exactly to JSON schema (0.0 to 1.0)
    pub schema_validity: f64,
    /// Rate at which the agent recovers from compiler or sandbox errors on subsequent attempts
    pub recovery_efficiency: f64,
    /// Estimated dollar cost per successfully verified task
    pub cost_per_success: f64,
}

/// Metric scores obtained on a benchmark suite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScore {
    pub total_tasks: usize,
    pub passed_tasks: usize,
    pub accuracy: f64,
    pub avg_latency_ms: f64,
    pub avg_tokens_used: usize,
    #[serde(default)]
    pub invariants: Option<StepInvariantMetrics>,
}

/// Generic trait defining a standardized agent evaluation suite.
#[async_trait]
pub trait BenchmarkSuite: Send + Sync {
    fn suite_name(&self) -> &str;
    async fn run_eval(&self) -> Result<BenchmarkScore>;
}
