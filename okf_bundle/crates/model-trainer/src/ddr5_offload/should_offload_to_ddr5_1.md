---
okf_version: "0.2"
type: Function
title: should_offload_to_ddr5
description: Check if GPU VRAM pressure requires offload to DDR5 RAM.
resource: crates/model-trainer/src/ddr5_offload.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/ddr5_offload/should_offload_to_ddr5_1
language: rust
---

# should_offload_to_ddr5

Check if GPU VRAM pressure requires offload to DDR5 RAM.

## Signature

```rust
pub fn should_offload_to_ddr5(&self, free_vram_mb: u64) -> bool
```

## Visibility

- `pub`

## Docstring

Check if GPU VRAM pressure requires offload to DDR5 RAM.

## Source
Lines 82–84 in `crates/model-trainer/src/ddr5_offload.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ddr5_offload](/crates/model-trainer/src/ddr5_offload.md) |
