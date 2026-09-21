use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionSlot {
    SlotA,
    SlotB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentState {
    Idle,
    Staged {
        slot: PartitionSlot,
        address: u32,
    },
    BootVerifying {
        slot: PartitionSlot,
        started_at_ms: i64,
        timeout_ms: u64,
    },
    ConfirmedHealthy {
        slot: PartitionSlot,
    },
    RolledBack {
        failed_slot: PartitionSlot,
        active_slot: PartitionSlot,
        reason: RollbackReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackReason {
    BootTimeout,
    HardFaultDetected,
    PanicDetected,
    WatchdogTrip,
}

#[derive(Debug, Error)]
pub enum AtomicFlashError {
    #[error("Target is not in a valid state for staging: {0:?}")]
    InvalidState(DeploymentState),
    #[error("Flash write error at address {0:#010X}: {1}")]
    FlashWriteFailed(u32, String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
}

/// A/B Partition Layout for STM32 / ARM Cortex-M
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionLayout {
    pub slot_a_base: u32, // e.g. 0x08008000 (32KB offset after bootloader)
    pub slot_b_base: u32, // e.g. 0x08080000 (512KB offset)
    pub slot_size_bytes: u32,
    pub bootloader_base: u32, // e.g. 0x08000000
}

impl Default for PartitionLayout {
    fn default() -> Self {
        Self {
            slot_a_base: 0x08008000,
            slot_b_base: 0x08080000,
            slot_size_bytes: 480 * 1024,
            bootloader_base: 0x08000000,
        }
    }
}

/// Atomic Flashing & Sentinel Rollback Manager
pub struct AtomicFlashManager {
    layout: PartitionLayout,
    active_slot: PartitionSlot,
    state: DeploymentState,
}

impl Default for AtomicFlashManager {
    fn default() -> Self {
        Self::new(PartitionLayout::default())
    }
}

impl AtomicFlashManager {
    pub fn new(layout: PartitionLayout) -> Self {
        Self {
            layout,
            active_slot: PartitionSlot::SlotA,
            state: DeploymentState::Idle,
        }
    }

    pub fn current_state(&self) -> DeploymentState {
        self.state
    }

    pub fn active_slot(&self) -> PartitionSlot {
        self.active_slot
    }

    /// Determine which partition should receive the candidate build
    pub fn get_candidate_slot(&self) -> (PartitionSlot, u32) {
        match self.active_slot {
            PartitionSlot::SlotA => (PartitionSlot::SlotB, self.layout.slot_b_base),
            PartitionSlot::SlotB => (PartitionSlot::SlotA, self.layout.slot_a_base),
        }
    }

    /// Stage new firmware in the alternate partition
    pub fn stage_firmware(
        &mut self,
        binary_len: usize,
    ) -> Result<(PartitionSlot, u32), AtomicFlashError> {
        if binary_len as u32 > self.layout.slot_size_bytes {
            return Err(AtomicFlashError::VerificationFailed(format!(
                "Binary size ({} bytes) exceeds slot size ({} bytes)",
                binary_len, self.layout.slot_size_bytes
            )));
        }

        let (slot, address) = self.get_candidate_slot();
        self.state = DeploymentState::Staged { slot, address };
        Ok((slot, address))
    }

    /// Instruct bootloader / target to switch vector to candidate and start verification window
    pub fn start_boot_verification(
        &mut self,
        timeout: Duration,
    ) -> Result<PartitionSlot, AtomicFlashError> {
        match self.state {
            DeploymentState::Staged { slot, .. } => {
                let now = chrono::Utc::now().timestamp_millis();
                self.state = DeploymentState::BootVerifying {
                    slot,
                    started_at_ms: now,
                    timeout_ms: timeout.as_millis() as u64,
                };
                Ok(slot)
            }
            other => Err(AtomicFlashError::InvalidState(other)),
        }
    }

    /// Target successfully confirmed boot & heartbeats; commit new slot as active
    pub fn confirm_healthy(&mut self) -> Result<PartitionSlot, AtomicFlashError> {
        match self.state {
            DeploymentState::BootVerifying { slot, .. } => {
                self.active_slot = slot;
                self.state = DeploymentState::ConfirmedHealthy { slot };
                Ok(slot)
            }
            other => Err(AtomicFlashError::InvalidState(other)),
        }
    }

    /// Trigger immediate fallback to known-good partition upon HardFault, panic or timeout
    pub fn rollback(&mut self, reason: RollbackReason) -> (PartitionSlot, u32) {
        let (failed_slot, fallback_slot) = match self.state {
            DeploymentState::BootVerifying { slot, .. } | DeploymentState::Staged { slot, .. } => {
                let fallback = match slot {
                    PartitionSlot::SlotA => PartitionSlot::SlotB,
                    PartitionSlot::SlotB => PartitionSlot::SlotA,
                };
                (slot, fallback)
            }
            _ => (self.get_candidate_slot().0, self.active_slot),
        };

        self.state = DeploymentState::RolledBack {
            failed_slot,
            active_slot: fallback_slot,
            reason,
        };
        self.active_slot = fallback_slot;

        let address = match fallback_slot {
            PartitionSlot::SlotA => self.layout.slot_a_base,
            PartitionSlot::SlotB => self.layout.slot_b_base,
        };

        tracing::warn!(
            target: "atomic_flash",
            "Target rolled back from {:?} to {:?} at address {:#010X} (Reason: {:?})",
            failed_slot, fallback_slot, address, reason
        );

        (fallback_slot, address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_flash_lifecycle_success() {
        let mut manager = AtomicFlashManager::default();
        assert_eq!(manager.active_slot(), PartitionSlot::SlotA);

        // Stage candidate on Slot B
        let (slot, addr) = manager.stage_firmware(1024).unwrap();
        assert_eq!(slot, PartitionSlot::SlotB);
        assert_eq!(addr, 0x08080000);

        // Start boot verification
        let verifying_slot = manager
            .start_boot_verification(Duration::from_secs(2))
            .unwrap();
        assert_eq!(verifying_slot, PartitionSlot::SlotB);

        // Health confirmed
        let confirmed = manager.confirm_healthy().unwrap();
        assert_eq!(confirmed, PartitionSlot::SlotB);
        assert_eq!(manager.active_slot(), PartitionSlot::SlotB);
    }

    #[test]
    fn test_atomic_flash_auto_rollback_on_hardfault() {
        let mut manager = AtomicFlashManager::default();
        let _ = manager.stage_firmware(2048).unwrap();
        let _ = manager
            .start_boot_verification(Duration::from_secs(2))
            .unwrap();

        // Target crashes immediately
        let (fallback_slot, fallback_addr) = manager.rollback(RollbackReason::HardFaultDetected);
        assert_eq!(fallback_slot, PartitionSlot::SlotA);
        assert_eq!(fallback_addr, 0x08008000);
        assert_eq!(manager.active_slot(), PartitionSlot::SlotA);

        if let DeploymentState::RolledBack { reason, .. } = manager.current_state() {
            assert_eq!(reason, RollbackReason::HardFaultDetected);
        } else {
            panic!("Expected RolledBack state");
        }
    }
}
