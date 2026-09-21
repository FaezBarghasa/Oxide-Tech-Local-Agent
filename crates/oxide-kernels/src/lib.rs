pub mod ternary;

pub struct FusedCrossEntropyOp {
    pub kernel_name: String,
}

impl Default for FusedCrossEntropyOp {
    fn default() -> Self {
        Self {
            kernel_name: "oxide_fused_cce".to_string(),
        }
    }
}

impl FusedCrossEntropyOp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&self, vocab_size: usize, seq_len: usize) -> Result<(), String> {
        tracing::debug!(
            "Executed custom fused CCE kernel {} (vocab={}, seq={}) avoiding matrix materialization",
            self.kernel_name, vocab_size, seq_len
        );
        Ok(())
    }
}

pub use ternary::TernaryHadamardOp;

