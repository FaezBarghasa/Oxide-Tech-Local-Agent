use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTrace {
    pub suite: String,
    pub task_id: String,
    pub steps_taken: usize,
    pub passed: bool,
    pub tool_calls: Vec<String>,
    pub timestamp: i64,
}

pub struct TraceLogger {
    log_dir: PathBuf,
}

impl TraceLogger {
    pub fn new(log_dir: PathBuf) -> Self {
        Self { log_dir }
    }

    pub async fn log_trace(&self, trace: &BenchmarkTrace) -> anyhow::Result<()> {
        fs::create_dir_all(&self.log_dir).await?;
        let trace_file = self.log_dir.join("benchmark_traces.jsonl");

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&trace_file)
            .await?;

        let serialized = serde_json::to_string(trace)?;
        file.write_all(serialized.as_bytes()).await?;
        file.write_all(b"\n").await?;
        Ok(())
    }
}
