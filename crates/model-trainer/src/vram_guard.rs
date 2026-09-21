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
    /// VRAM exhausted; dynamically spill weights/activations over PCIe into host DDR5 RAM.
    OffloadToDdr5 { required_offload_mb: u64 },
    /// Critical limit reached (<500MB free) and DDR5 fallback exhausted; halt iteration.
    AbortOomImminent,
}

/// Active telemetry supervisor monitoring RAM and VRAM allocation budgets.
pub struct VramGuard {
    min_headroom_mb: u64,
    checkpointing_threshold_mb: u64,
    ddr5_spillover_threshold_mb: u64,
}

impl Default for VramGuard {
    fn default() -> Self {
        Self {
            min_headroom_mb: 400,             // Critical cutoff
            ddr5_spillover_threshold_mb: 800, // Trigger DDR5 offloading if < 800MB
            checkpointing_threshold_mb: 1800, // Trigger checkpointing if < 1.8GB
        }
    }
}

impl VramGuard {
    pub fn new(
        min_headroom_mb: u64,
        ddr5_spillover_threshold_mb: u64,
        checkpointing_threshold_mb: u64,
    ) -> Self {
        Self {
            min_headroom_mb,
            ddr5_spillover_threshold_mb,
            checkpointing_threshold_mb,
        }
    }

    /// Read available memory on the host system in Megabytes.
    pub fn available_system_memory_mb() -> u64 {
        let mut sys = System::new();
        sys.refresh_memory();
        sys.available_memory() / (1024 * 1024)
    }

    /// Evaluate current VRAM / memory headroom and prescribe the optimal defense action,
    /// seamlessly routing allocations to host DDR5 RAM before aborting.
    pub fn evaluate_headroom(&self, free_vram_mb: u64, current_batch: u32) -> VramAction {
        if free_vram_mb < self.min_headroom_mb {
            let host_free_mb = Self::available_system_memory_mb();
            if host_free_mb >= 2048 {
                tracing::info!(
                    target: "vram_guard",
                    "Critical VRAM ({} MB free). Spilling tensor buffers to available host DDR5 RAM ({} MB free)",
                    free_vram_mb,
                    host_free_mb
                );
                VramAction::OffloadToDdr5 {
                    required_offload_mb: self.min_headroom_mb.saturating_sub(free_vram_mb) + 1024,
                }
            } else {
                tracing::warn!(
                    target: "vram_guard",
                    "Critical VRAM & DDR5 exhaustion: {} MB VRAM, {} MB DDR5 free",
                    free_vram_mb,
                    host_free_mb
                );
                VramAction::AbortOomImminent
            }
        } else if free_vram_mb < self.ddr5_spillover_threshold_mb {
            let host_free_mb = Self::available_system_memory_mb();
            if host_free_mb >= 2048 {
                VramAction::OffloadToDdr5 {
                    required_offload_mb: self
                        .ddr5_spillover_threshold_mb
                        .saturating_sub(free_vram_mb),
                }
            } else if current_batch > 1 {
                VramAction::ReduceBatchSize {
                    suggested_batch: (current_batch / 2).max(1),
                }
            } else {
                VramAction::EnableGradientCheckpointing
            }
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
        let guard = VramGuard::new(400, 800, 1800);

        // Plenty of headroom
        assert_eq!(guard.evaluate_headroom(4000, 4), VramAction::Proceed);

        // Constrained VRAM (1200MB) with batch size 4 -> reduce batch size
        assert_eq!(
            guard.evaluate_headroom(1200, 4),
            VramAction::ReduceBatchSize { suggested_batch: 2 }
        );

        // Under spillover threshold (<800MB) -> trigger DDR5 host offload if RAM available
        let action = guard.evaluate_headroom(600, 2);
        match action {
            VramAction::OffloadToDdr5 {
                required_offload_mb,
            } => {
                assert!(required_offload_mb > 0);
            }
            other => panic!("Expected OffloadToDdr5, got {:?}", other),
        }
    }
}
