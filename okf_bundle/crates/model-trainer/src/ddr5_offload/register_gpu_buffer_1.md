---
okf_version: "0.2"
type: Function
title: register_gpu_buffer
description: Register a buffer in GPU VRAM
resource: crates/model-trainer/src/ddr5_offload.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/ddr5_offload/register_gpu_buffer_1
language: rust
---

# register_gpu_buffer

Register a buffer in GPU VRAM

## Signature

```rust
pub fn register_gpu_buffer(&self, name: &str, size_bytes: usize, device_ptr: u64)
```

## Visibility

- `pub`

## Docstring

Register a buffer in GPU VRAM

## Source
Lines 67–79 in `crates/model-trainer/src/ddr5_offload.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ddr5_offload](/crates/model-trainer/src/ddr5_offload.md) |
