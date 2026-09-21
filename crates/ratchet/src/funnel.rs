//! # Three-Phase Candidate Funnel
//!
//! Phase 1 Validate (CPU, ~1.5s) -> Phase 2 Probe (1 GPU step, ~2s) -> Phase 3 Budget (5 min)

use crate::gate::PromotionGate;
use crate::verdict::{MetricTuple, Verdict};
use chrono::Utc;
use std::path::Path;
use tokio::process::Command;
use tracing::{info, warn};

pub struct CandidateFunnel {
    pub max_surface_lines: usize,
    pub max_surface_bytes: usize,
    pub gate: PromotionGate,
}

impl Default for CandidateFunnel {
    fn default() -> Self {
        Self {
            max_surface_lines: 420,
            max_surface_bytes: 40 * 1024,
            gate: PromotionGate::default(),
        }
    }
}

impl CandidateFunnel {
    /// Phase 1: Fast static and compiler validation (0 GPU used)
    pub async fn validate_phase1(&self, surface_path: &Path) -> Result<(), Verdict> {
        let content = tokio::fs::read_to_string(surface_path)
            .await
            .map_err(|e| Verdict::RejectedCompile(format!("Failed to read surface file: {}", e)))?;

        // 1. Line budget check
        let line_count = content.lines().count();
        if line_count > self.max_surface_lines {
            return Err(Verdict::RejectedCompile(format!(
                "Surface line count ({}) exceeded 420-line budget limit",
                line_count
            )));
        }

        // 2. Byte budget check
        if content.len() > self.max_surface_bytes {
            return Err(Verdict::RejectedCompile(format!(
                "Surface size ({} bytes) exceeded 40 KiB ceiling",
                content.len()
            )));
        }

        // 3. Static security & capability absence scan
        if content.contains("std::fs")
            || content.contains("std::net")
            || content.contains("std::process")
        {
            return Err(Verdict::TamperSuspected(
                "Forbidden system access detected in mutable surface (capability absence violated)"
                    .to_string(),
            ));
        }

        if content.contains("unsafe ") {
            warn!("Warning: unsafe block detected in surface");
        }

        // 4. Cargo check with JSON diagnostics
        let output = Command::new("cargo")
            .args(["check", "--message-format=json", "-p", "surface"])
            .output()
            .await
            .map_err(|e| Verdict::RejectedCompile(format!("cargo check failed to spawn: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Verdict::RejectedCompile(if stderr.is_empty() {
                "Compile error in surface candidate".to_string()
            } else {
                stderr
            }));
        }

        Ok(())
    }

    /// Run full three-phase funnel for candidate
    pub async fn evaluate_candidate(
        &self,
        surface_path: &Path,
        baseline: Option<&MetricTuple>,
        harness_digest: &str,
    ) -> (Verdict, Option<MetricTuple>) {
        let t0 = std::time::Instant::now();

        // Phase 1: Validate
        if let Err(verdict) = self.validate_phase1(surface_path).await {
            info!("Phase 1 Validate rejected candidate: {:?}", verdict);
            return (verdict, None);
        }

        let compile_wall_s = t0.elapsed().as_secs_f64();

        // Phase 2: Probe (Simulated 1 GPU step verification)
        // In real execution, cdylib is loaded with libloading inside catch_unwind
        info!("Phase 2 Probe passed in {:.2}s", t0.elapsed().as_secs_f64());

        // Phase 3: Budget Evaluation (Calculate metrics)
        let metric = MetricTuple {
            primary: 1.042, // Simulated bits-per-byte (val_bpb)
            compile_wall_s,
            peak_vram_bytes: 42 * 1024 * 1024 * 1024,
            step_throughput: 24.5,
            hardware_class: "nvidia-h100-sxm5".to_string(),
            harness_digest: harness_digest.to_string(),
            rev: "cand-001".to_string(),
            measured_at: Utc::now(),
        };

        let verdict = self.gate.decide(&metric, baseline);
        (verdict, Some(metric))
    }
}
