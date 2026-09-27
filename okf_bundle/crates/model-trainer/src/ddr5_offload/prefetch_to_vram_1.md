---
okf_version: "0.2"
type: Function
title: prefetch_to_vram
description: Prefetch/stream a buffer from DDR5 RAM back into GPU VRAM for computation.
resource: crates/model-trainer/src/ddr5_offload.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/ddr5_offload/prefetch_to_vram_1
language: rust
---

# prefetch_to_vram

Prefetch/stream a buffer from DDR5 RAM back into GPU VRAM for computation.

## Signature

```rust
pub fn prefetch_to_vram(
        &self,
        name: &str,
        new_device_ptr: u64,
    ) -> Result<Vec<u8>, String>
```

## Visibility

- `pub`

## Docstring

Prefetch/stream a buffer from DDR5 RAM back into GPU VRAM for computation.

## Source
Lines 137–167 in `crates/model-trainer/src/ddr5_offload.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ddr5_offload](/crates/model-trainer/src/ddr5_offload.md) |
