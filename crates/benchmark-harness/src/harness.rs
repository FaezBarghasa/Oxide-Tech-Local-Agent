use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Metric scores obtained on a benchmark suite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScore {
    pub total_tasks: usize,
    pub passed_tasks: usize,
    pub accuracy: f64,
    pub avg_latency_ms: f64,
    pub avg_tokens_used: usize,
}

/// Generic trait defining a standardized agent evaluation suite.
#[async_trait]
pub trait BenchmarkSuite: Send + Sync {
    fn suite_name(&self) -> &str;
    async fn run_eval(&self) -> Result<BenchmarkScore>;
}
