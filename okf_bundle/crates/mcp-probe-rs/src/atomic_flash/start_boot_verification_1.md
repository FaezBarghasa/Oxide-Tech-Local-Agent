---
okf_version: "0.2"
type: Function
title: start_boot_verification
description: Instruct bootloader / target to switch vector to candidate and start verification window
resource: crates/mcp-probe-rs/src/atomic_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/atomic_flash/start_boot_verification_1
language: rust
---

# start_boot_verification

Instruct bootloader / target to switch vector to candidate and start verification window

## Signature

```rust
pub fn start_boot_verification(
        &mut self,
        timeout: Duration,
    ) -> Result<PartitionSlot, AtomicFlashError>
```

## Visibility

- `pub`

## Docstring

Instruct bootloader / target to switch vector to candidate and start verification window

## Source
Lines 127–143 in `crates/mcp-probe-rs/src/atomic_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_flash](/crates/mcp-probe-rs/src/atomic_flash.md) |
