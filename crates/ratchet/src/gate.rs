//! # Promotion Gate
//!
//! Evaluates candidate metric tuples against baseline with strict guard metric
//! constraints, hardware pinning, and digest verification.

use crate::verdict::{MetricTuple, Verdict};

pub struct PromotionGate {
    pub max_compile_wall_s: f64,
    pub max_peak_vram_bytes: u64,
    pub require_identical_harness_digest: bool,
    pub pin_hardware_class: bool,
}

impl Default for PromotionGate {
    fn default() -> Self {
        Self {
            max_compile_wall_s: 20.0,
            max_peak_vram_bytes: 80 * 1024 * 1024 * 1024, // 80 GiB
            require_identical_harness_digest: true,
            pin_hardware_class: true,
        }
    }
}

impl PromotionGate {
    pub fn decide(&self, candidate: &MetricTuple, baseline: Option<&MetricTuple>) -> Verdict {
        // 1. Guard check: Compile latency ceiling
        if candidate.compile_wall_s > self.max_compile_wall_s {
            return Verdict::RejectedGate(format!(
                "Compile latency ({:.2}s) exceeded guard ceiling of {:.2}s (TooHeavy)",
                candidate.compile_wall_s, self.max_compile_wall_s
            ));
        }

        // 2. Guard check: Peak VRAM ceiling
        if candidate.peak_vram_bytes > self.max_peak_vram_bytes {
            return Verdict::RejectedGate(format!(
                "Peak VRAM ({} bytes) exceeded ceiling of {} bytes",
                candidate.peak_vram_bytes, self.max_peak_vram_bytes
            ));
        }

        let Some(base) = baseline else {
            // First candidate establishes baseline
            return Verdict::Kept;
        };

        // 3. Harness Digest Equality check
        if self.require_identical_harness_digest && candidate.harness_digest != base.harness_digest
        {
            return Verdict::RejectedGate(format!(
                "Harness digest mismatch: candidate '{}' vs baseline '{}'",
                candidate.harness_digest, base.harness_digest
            ));
        }

        // 4. Hardware Pinning check
        if self.pin_hardware_class && candidate.hardware_class != base.hardware_class {
            return Verdict::RejectedGate(format!(
                "Hardware class mismatch: candidate '{}' vs baseline '{}'",
                candidate.hardware_class, base.hardware_class
            ));
        }

        // 5. Scientific Metric Improvement (Lower is better for val_bpb / loss)
        if candidate.primary < base.primary {
            Verdict::Kept
        } else {
            Verdict::Reverted
        }
    }
}
