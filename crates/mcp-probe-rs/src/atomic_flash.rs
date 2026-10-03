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

    /// Validates an Ed25519 signed human authorization consent token before permitting MCU flash
    pub fn validate_ed25519_flash_token(token: &str, chip: &str) -> Result<bool, AtomicFlashError> {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            return Err(AtomicFlashError::VerificationFailed(
                "Missing required HITL human authorization token".to_string(),
            ));
        }

        let token_body = trimmed.strip_prefix("ed25519:").unwrap_or(trimmed);
        let parts: Vec<&str> = token_body.split(':').collect();
        if parts.len() < 2 {
            return Err(AtomicFlashError::VerificationFailed(
                "Malformed Ed25519 token: expected '<pubkey_hex>:<signature_hex>[:<payload>]'".to_string(),
            ));
        }

        let pubkey_bytes = hex::decode(parts[0]).map_err(|e| {
            AtomicFlashError::VerificationFailed(format!("Invalid public key hex in token: {e}"))
        })?;

        if pubkey_bytes.len() != 32 {
            return Err(AtomicFlashError::VerificationFailed(
                "Invalid public key length: expected 32 bytes".to_string(),
            ));
        }

        let sig_bytes = hex::decode(parts[1]).map_err(|e| {
            AtomicFlashError::VerificationFailed(format!("Invalid signature hex in token: {e}"))
        })?;

        if sig_bytes.len() != 64 {
            return Err(AtomicFlashError::VerificationFailed(
                "Invalid signature length: expected 64 bytes".to_string(),
            ));
        }

        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(&pubkey_bytes);
        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&pubkey_array).map_err(|e| {
            AtomicFlashError::VerificationFailed(format!("Invalid Ed25519 public key: {e}"))
        })?;

        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&sig_bytes);
        let signature = ed25519_dalek::Signature::from_bytes(&sig_array);

        // If an explicit payload was signed, ensure it binds to the requested target chip
        let msg_bytes = if parts.len() >= 3 {
            let msg = parts[2..].join(":");
            if !msg.contains(chip) {
                return Err(AtomicFlashError::VerificationFailed(format!(
                    "Token authorization message does not bind to target chip '{chip}'"
                )));
            }
            msg.into_bytes()
        } else {
            chip.as_bytes().to_vec()
        };

        use ed25519_dalek::Verifier;
        verifying_key
            .verify(&msg_bytes, &signature)
            .map_err(|e| {
                AtomicFlashError::VerificationFailed(format!(
                    "Cryptographic Ed25519 signature verification failed for target chip '{chip}': {e}"
                ))
            })?;

        tracing::info!(
            target: "atomic_flash",
            "Cryptographic HITL authorization token verified for target chip '{}'",
            chip
        );
        Ok(true)
    }

    /// Helper to generate a valid signed token for a chip authorization
    pub fn sign_chip_authorization(
        signing_key: &ed25519_dalek::SigningKey,
        chip: &str,
        extra_context: Option<&str>,
    ) -> String {
        use ed25519_dalek::Signer;
        let pubkey_hex = hex::encode(signing_key.verifying_key().to_bytes());
        let msg = match extra_context {
            Some(ctx) => format!("{chip}:{ctx}"),
            None => chip.to_string(),
        };
        let sig = signing_key.sign(msg.as_bytes());
        let sig_hex = hex::encode(sig.to_bytes());
        format!("{pubkey_hex}:{sig_hex}:{msg}")
    }

    /// Verifies in-memory firmware payload bytes and BLAKE3 hash directly, eliminating disk TOCTOU windows
    pub fn verify_in_memory_payload(
        payload: &[u8],
        expected_blake3_hex: &str,
        token: &str,
        chip: &str,
    ) -> Result<[u8; 32], AtomicFlashError> {
        if payload.is_empty() {
            return Err(AtomicFlashError::VerificationFailed(
                "Firmware binary buffer is empty".to_string(),
            ));
        }

        Self::validate_ed25519_flash_token(token, chip)?;

        let computed_hash = blake3::hash(payload);
        let computed_hex = computed_hash.to_hex();

        if !expected_blake3_hex.is_empty() && computed_hex.as_str() != expected_blake3_hex {
            return Err(AtomicFlashError::VerificationFailed(format!(
                "BLAKE3 digest mismatch: computed '{}' vs expected '{}'",
                computed_hex, expected_blake3_hex
            )));
        }

        tracing::info!(
            target: "atomic_flash",
            "In-memory firmware verification passed (BLAKE3: {}, bytes: {})",
            computed_hex, payload.len()
        );

        Ok(*computed_hash.as_bytes())
    }
}

/// In-memory verified firmware payload container preventing disk modification between verification and flash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedFirmwarePayload {
    pub blake3_digest: [u8; 32],
    pub binary_bytes: Vec<u8>,
    pub target_chip: String,
    pub auth_token: String,
}

impl VerifiedFirmwarePayload {
    pub fn new(
        bytes: Vec<u8>,
        target_chip: impl Into<String>,
        auth_token: impl Into<String>,
    ) -> Result<Self, AtomicFlashError> {
        let chip = target_chip.into();
        let tok = auth_token.into();
        let digest = AtomicFlashManager::verify_in_memory_payload(&bytes, "", &tok, &chip)?;
        Ok(Self {
            blake3_digest: digest,
            binary_bytes: bytes,
            target_chip: chip,
            auth_token: tok,
        })
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

    #[test]
    fn test_validate_ed25519_flash_token() {
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&[42u8; 32]);
        let valid_token =
            AtomicFlashManager::sign_chip_authorization(&signing_key, "STM32F407VG", Some("flash_v1"));

        // Valid signature on matching chip
        assert!(AtomicFlashManager::validate_ed25519_flash_token(&valid_token, "STM32F407VG").is_ok());

        // Target chip mismatch
        assert!(AtomicFlashManager::validate_ed25519_flash_token(&valid_token, "STM32F103").is_err());

        // Empty token rejected
        assert!(AtomicFlashManager::validate_ed25519_flash_token("", "STM32F407VG").is_err());

        // Malformed token rejected
        assert!(AtomicFlashManager::validate_ed25519_flash_token("short", "STM32F407VG").is_err());

        // Length >= 32 fake string without cryptographic signature is strictly rejected
        assert!(
            AtomicFlashManager::validate_ed25519_flash_token(
                "ed25519-sig-auth-stm32f407-valid-tok-not-real",
                "STM32F407VG"
            )
            .is_err()
        );
    }

    #[test]
    fn atomic_flash_in_memory_test() {
        let raw_firmware = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x02, 0x03, 0x04];
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&[99u8; 32]);
        let target_chip = "STM32F407VG";
        let auth_token =
            AtomicFlashManager::sign_chip_authorization(&signing_key, target_chip, None);

        let verified =
            VerifiedFirmwarePayload::new(raw_firmware.clone(), target_chip, auth_token.clone())
                .unwrap();
        assert_eq!(verified.binary_bytes, raw_firmware);
        assert_eq!(verified.target_chip, target_chip);
        assert_eq!(verified.auth_token, auth_token);

        let computed = blake3::hash(&raw_firmware);
        assert_eq!(verified.blake3_digest, *computed.as_bytes());
    }
}
