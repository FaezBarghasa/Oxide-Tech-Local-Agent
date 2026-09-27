---
okf_version: "0.2"
type: Function
title: verify_rklipper_firmware
description: Emulates r-klipper firmware ELF binary on a QEMU Cortex-M4 target (e.g. Netduino / STM32)
resource: crates/verifier/src/qemu_firmware.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/qemu_firmware/verify_rklipper_firmware
language: rust
---

# verify_rklipper_firmware

Emulates r-klipper firmware ELF binary on a QEMU Cortex-M4 target (e.g. Netduino / STM32)

## Signature

```rust
impl FirmwareEmulationVerifier { pub fn verify_rklipper_firmware(
        &self,
        elf_path: &PathBuf,
        target_board: &str, // e.g. "netduinoplus2" or "lm3s6965evb"
    ) -> Result<bool> }
```

## Visibility

- `pub`

## Docstring

Emulates r-klipper firmware ELF binary on a QEMU Cortex-M4 target (e.g. Netduino / STM32)

## Source
Lines 27–93 in `crates/verifier/src/qemu_firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qemu_firmware](/crates/verifier/src/qemu_firmware.md) |
