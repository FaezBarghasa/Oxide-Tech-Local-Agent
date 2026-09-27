---
okf_version: "0.2"
type: Class
title: VramGuard
description: Active telemetry supervisor monitoring RAM and VRAM allocation budgets.
resource: crates/model-trainer/src/vram_guard.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/vram_guard/VramGuard
language: rust
---

# VramGuard

Active telemetry supervisor monitoring RAM and VRAM allocation budgets.

## Signature

```rust
pub struct VramGuard
```

## Visibility

- `pub`

## Docstring

Active telemetry supervisor monitoring RAM and VRAM allocation budgets.

## Methods

- `min_headroom_mb`
- `checkpointing_threshold_mb`
- `ddr5_spillover_threshold_mb`

## Source
Lines 19–23 in `crates/model-trainer/src/vram_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vram_guard](/crates/model-trainer/src/vram_guard.md) |
