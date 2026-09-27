---
okf_version: "0.2"
type: Function
title: confirm_healthy
description: "Target successfully confirmed boot & heartbeats; commit new slot as active"
resource: crates/mcp-probe-rs/src/atomic_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/atomic_flash/confirm_healthy
language: rust
---

# confirm_healthy

Target successfully confirmed boot & heartbeats; commit new slot as active

## Signature

```rust
impl AtomicFlashManager { pub fn confirm_healthy(&mut self) -> Result<PartitionSlot, AtomicFlashError> }
```

## Visibility

- `pub`

## Docstring

Target successfully confirmed boot & heartbeats; commit new slot as active

## Source
Lines 146–155 in `crates/mcp-probe-rs/src/atomic_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_flash](/crates/mcp-probe-rs/src/atomic_flash.md) |
