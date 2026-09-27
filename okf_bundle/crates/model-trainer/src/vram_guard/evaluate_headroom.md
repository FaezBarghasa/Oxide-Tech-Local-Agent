---
okf_version: "0.2"
type: Function
title: evaluate_headroom
description: "Evaluate current VRAM / memory headroom and prescribe the optimal defense action,"
resource: crates/model-trainer/src/vram_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/vram_guard/evaluate_headroom
language: rust
---

# evaluate_headroom

Evaluate current VRAM / memory headroom and prescribe the optimal defense action,

## Signature

```rust
impl VramGuard { pub fn evaluate_headroom(&self, free_vram_mb: u64, current_batch: u32) -> VramAction }
```

## Visibility

- `pub`

## Docstring

Evaluate current VRAM / memory headroom and prescribe the optimal defense action,
seamlessly routing allocations to host DDR5 RAM before aborting.

## Source
Lines 57–118 in `crates/model-trainer/src/vram_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vram_guard](/crates/model-trainer/src/vram_guard.md) |
