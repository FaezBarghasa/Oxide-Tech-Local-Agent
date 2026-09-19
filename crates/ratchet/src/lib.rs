//! # Ratchet
//!
//! Three-phase candidate evaluation funnel, fail-fast verification, and promotion gate
//! for the Oxide-Tech Autoresearch pure-Rust training surface.

pub mod funnel;
pub mod gate;
pub mod verdict;

pub use funnel::CandidateFunnel;
pub use gate::PromotionGate;
pub use verdict::{MetricTuple, Verdict};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_promotion_gate_decisions() {
        let gate = PromotionGate::default();

        let baseline = MetricTuple {
            primary: 1.050,
            compile_wall_s: 2.1,
            peak_vram_bytes: 40 * 1024 * 1024 * 1024,
            step_throughput: 20.0,
            hardware_class: "nvidia-h100-sxm5".to_string(),
            harness_digest: "DIGEST_AAA".to_string(),
            rev: "base".to_string(),
            measured_at: Utc::now(),
        };

        // 1. Better primary metric -> Kept
        let cand_better = MetricTuple {
            primary: 1.040,
            compile_wall_s: 2.5,
            peak_vram_bytes: 40 * 1024 * 1024 * 1024,
            step_throughput: 20.0,
            hardware_class: "nvidia-h100-sxm5".to_string(),
            harness_digest: "DIGEST_AAA".to_string(),
            rev: "cand-better".to_string(),
            measured_at: Utc::now(),
        };
        assert_eq!(gate.decide(&cand_better, Some(&baseline)), Verdict::Kept);

        // 2. Worse primary metric -> Reverted
        let cand_worse = MetricTuple {
            primary: 1.060,
            ..cand_better.clone()
        };
        assert_eq!(gate.decide(&cand_worse, Some(&baseline)), Verdict::Reverted);

        // 3. Compile latency exceeded -> RejectedGate
        let cand_slow_compile = MetricTuple {
            compile_wall_s: 25.0, // > 20s
            ..cand_better.clone()
        };
        assert!(matches!(
            gate.decide(&cand_slow_compile, Some(&baseline)),
            Verdict::RejectedGate(_)
        ));

        // 4. Harness digest mismatch -> RejectedGate
        let cand_digest_mismatch = MetricTuple {
            harness_digest: "DIGEST_BBB".to_string(),
            ..cand_better.clone()
        };
        assert!(matches!(
            gate.decide(&cand_digest_mismatch, Some(&baseline)),
            Verdict::RejectedGate(_)
        ));
    }
}
