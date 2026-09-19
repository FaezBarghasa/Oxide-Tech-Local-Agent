//! # Verdict Taxonomy and Metric Tuple
//!
//! Distinct failure verdicts prevent compiler, syntax, or runtime errors from
//! polluting scientific hypothesis revert metrics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Verdict {
    /// Real scientific win meeting all metric and guard thresholds
    Kept,
    /// Ran cleanly to completion, but scientific validation metric did not improve
    Reverted,
    /// Rustc / clippy / syntax / manifest / line-budget rejection (0 GPU wasted)
    RejectedCompile(String),
    /// Caught by catch_unwind or CPU dry-run before full budget (~1 step wasted)
    RejectedRuntime(String),
    /// Hard fault (SIGSEGV / CUDA poison / hardware crash) requiring sandbox reset
    RejectedFault(String),
    /// Ran clean, but violated a guard metric (compile latency, memory, digest mismatch)
    RejectedGate(String),
    /// Sentinel detected unauthorized file/network/process attempt
    TamperSuspected(String),
    /// Process crash
    Crashed(String),
    /// Execution exceeded timeout ceiling
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTuple {
    /// Primary scientific metric (e.g. bits-per-byte val_bpb, lower is better)
    pub primary: f64,
    /// Compilation wall clock time in seconds (guard metric)
    pub compile_wall_s: f64,
    /// Peak VRAM consumption in bytes
    pub peak_vram_bytes: u64,
    /// Step throughput (steps / second)
    pub step_throughput: f64,
    /// Hardware class pinning identifier (e.g. "nvidia-h100-sxm5")
    pub hardware_class: String,
    /// SHA256 digest of frozen harness + toolchain + ABI
    pub harness_digest: String,
    /// Git revision of candidate
    pub rev: String,
    /// Measurement timestamp
    pub measured_at: DateTime<Utc>,
}
