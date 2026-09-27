---
okf_version: "0.2"
type: Function
title: stage_firmware
description: Stage new firmware in the alternate partition
resource: crates/mcp-probe-rs/src/atomic_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/atomic_flash/stage_firmware
language: rust
---

# stage_firmware

Stage new firmware in the alternate partition

## Signature

```rust
impl AtomicFlashManager { pub fn stage_firmware(
        &mut self,
        binary_len: usize,
    ) -> Result<(PartitionSlot, u32), AtomicFlashError> }
```

## Visibility

- `pub`

## Docstring

Stage new firmware in the alternate partition

## Source
Lines 110–124 in `crates/mcp-probe-rs/src/atomic_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_flash](/crates/mcp-probe-rs/src/atomic_flash.md) |
