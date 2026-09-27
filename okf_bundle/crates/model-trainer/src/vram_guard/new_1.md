---
okf_version: "0.2"
type: Function
title: new
resource: crates/model-trainer/src/vram_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/vram_guard/new_1
language: rust
---

# new

## Signature

```rust
pub fn new(
        min_headroom_mb: u64,
        ddr5_spillover_threshold_mb: u64,
        checkpointing_threshold_mb: u64,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 36–46 in `crates/model-trainer/src/vram_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vram_guard](/crates/model-trainer/src/vram_guard.md) |
