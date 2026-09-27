---
okf_version: "0.2"
type: Function
title: rollback
description: "Trigger immediate fallback to known-good partition upon HardFault, panic or timeout"
resource: crates/mcp-probe-rs/src/atomic_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/atomic_flash/rollback_1
language: rust
---

# rollback

Trigger immediate fallback to known-good partition upon HardFault, panic or timeout

## Signature

```rust
pub fn rollback(&mut self, reason: RollbackReason) -> (PartitionSlot, u32)
```

## Visibility

- `pub`

## Docstring

Trigger immediate fallback to known-good partition upon HardFault, panic or timeout

## Source
Lines 158–189 in `crates/mcp-probe-rs/src/atomic_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_flash](/crates/mcp-probe-rs/src/atomic_flash.md) |
