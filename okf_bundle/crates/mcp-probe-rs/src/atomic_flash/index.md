# atomic_flash

## Classs

- [AtomicFlashError](AtomicFlashError.md) — [derive(Debug, Error)]
- [AtomicFlashManager](AtomicFlashManager.md) — Atomic Flashing & Sentinel Rollback Manager
- [DeploymentState](DeploymentState.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [PartitionLayout](PartitionLayout.md) — A/B Partition Layout for STM32 / ARM Cortex-M
- [PartitionSlot](PartitionSlot.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [RollbackReason](RollbackReason.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

## Functions

- [active_slot](active_slot.md)
- [active_slot](active_slot_1.md)
- [confirm_healthy](confirm_healthy.md) — Target successfully confirmed boot & heartbeats; commit new slot as active
- [confirm_healthy](confirm_healthy_1.md) — Target successfully confirmed boot & heartbeats; commit new slot as active
- [current_state](current_state.md)
- [current_state](current_state_1.md)
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [get_candidate_slot](get_candidate_slot.md) — Determine which partition should receive the candidate build
- [get_candidate_slot](get_candidate_slot_1.md) — Determine which partition should receive the candidate build
- [new](new.md)
- [new](new_1.md)
- [rollback](rollback.md) — Trigger immediate fallback to known-good partition upon HardFault, panic or timeout
- [rollback](rollback_1.md) — Trigger immediate fallback to known-good partition upon HardFault, panic or timeout
- [stage_firmware](stage_firmware.md) — Stage new firmware in the alternate partition
- [stage_firmware](stage_firmware_1.md) — Stage new firmware in the alternate partition
- [start_boot_verification](start_boot_verification.md) — Instruct bootloader / target to switch vector to candidate and start verification window
- [start_boot_verification](start_boot_verification_1.md) — Instruct bootloader / target to switch vector to candidate and start verification window
- [test_atomic_flash_auto_rollback_on_hardfault](test_atomic_flash_auto_rollback_on_hardfault.md) — [test]
- [test_atomic_flash_lifecycle_success](test_atomic_flash_lifecycle_success.md) — [test]
