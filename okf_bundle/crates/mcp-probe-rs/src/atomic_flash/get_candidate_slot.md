---
okf_version: "0.2"
type: Function
title: get_candidate_slot
description: Determine which partition should receive the candidate build
resource: crates/mcp-probe-rs/src/atomic_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/atomic_flash/get_candidate_slot
language: rust
---

# get_candidate_slot

Determine which partition should receive the candidate build

## Signature

```rust
impl AtomicFlashManager { pub fn get_candidate_slot(&self) -> (PartitionSlot, u32) }
```

## Visibility

- `pub`

## Docstring

Determine which partition should receive the candidate build

## Source
Lines 102–107 in `crates/mcp-probe-rs/src/atomic_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_flash](/crates/mcp-probe-rs/src/atomic_flash.md) |
