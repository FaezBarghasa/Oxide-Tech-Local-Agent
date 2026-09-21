use sysinfo::System;

/// Adaptive action prescribed by the VRAM Guard to prevent Out-Of-Memory aborts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VramAction {
    /// Safe headroom available; proceed with standard forward/backward pass.
    Proceed,
    /// Memory pressure detected; enable activation recomputation (gradient checkpointing).
    EnableGradientCheckpointing,
    /// High memory pressure; reduce batch size to prevent allocation faults.
    ReduceBatchSize { suggested_batch: u32 },
    /// Critical limit reached (<500MB free); halt iteration before CUDA driver panic.
    AbortOomImminent,
}

/// Active telemetry supervisor monitoring RAM and VRAM allocation budgets.
pub struct VramGuard {
    min_headroom_mb: u64,
    checkpointing_threshold_mb: u64,
}

impl Default for VramGuard {
    fn default() -> Self {
        Self {
            min_headroom_mb: 600,             // Abort if headroom drops below 600MB
            checkpointing_threshold_mb: 1800, // Trigger checkpointing if < 1.8GB
        }
    }
}

impl VramGuard {
    pub fn new(min_headroom_mb: u64, checkpointing_threshold_mb: u64) -> Self {
        Self {
            min_headroom_mb,
            checkpointing_threshold_mb,
        }
    }

    /// Read available memory on the host system in Megabytes.
    pub fn available_system_memory_mb() -> u64 {
        let mut sys = System::new();
        sys.refresh_memory();
        sys.available_memory() / (1024 * 1024)
    }

    /// Evaluate current VRAM / memory headroom and prescribe the optimal defense action.
    pub fn evaluate_headroom(&self, free_vram_mb: u64, current_batch: u32) -> VramAction {
        if free_vram_mb < self.min_headroom_mb {
            tracing::warn!(
                target: "vram_guard",
                "Critical VRAM exhaustion: {} MB free < {} MB threshold",
                free_vram_mb,
                self.min_headroom_mb
            );
            VramAction::AbortOomImminent
        } else if free_vram_mb < self.checkpointing_threshold_mb {
            if current_batch > 1 {
                let suggested = (current_batch / 2).max(1);
                tracing::info!(
                    target: "vram_guard",
                    "Constrained VRAM ({} MB free). Throttling batch size from {} to {}",
                    free_vram_mb,
                    current_batch,
                    suggested
                );
                VramAction::ReduceBatchSize {
                    suggested_batch: suggested,
                }
            } else {
                tracing::info!(
                    target: "vram_guard",
                    "Constrained VRAM ({} MB free). Triggering activation checkpointing",
                    free_vram_mb
                );
                VramAction::EnableGradientCheckpointing
            }
        } else {
            VramAction::Proceed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vram_guard_actions() {
        let guard = VramGuard::new(500, 1500);

        // Plenty of headroom
        assert_eq!(guard.evaluate_headroom(4000, 4), VramAction::Proceed);

        // Medium pressure with batch size 4 -> reduce batch size
        assert_eq!(
            guard.evaluate_headroom(1000, 4),
            VramAction::ReduceBatchSize { suggested_batch: 2 }
        );

        // Medium pressure with batch size 1 -> enable gradient checkpointing
        assert_eq!(
            guard.evaluate_headroom(1000, 1),
            VramAction::EnableGradientCheckpointing
        );

        // Critical limit -> abort
        assert_eq!(
            guard.evaluate_headroom(300, 1),
            VramAction::AbortOomImminent
        );
    }
}
