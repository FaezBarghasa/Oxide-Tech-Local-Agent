---
okf_version: "0.2"
type: Function
title: offload_to_ddr5
description: Evict/offload a buffer from GPU VRAM into pinned DDR5 host memory.
resource: crates/model-trainer/src/ddr5_offload.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/ddr5_offload/offload_to_ddr5_1
language: rust
---

# offload_to_ddr5

Evict/offload a buffer from GPU VRAM into pinned DDR5 host memory.

## Signature

```rust
pub fn offload_to_ddr5(
        &self,
        name: &str,
        data_from_device: Option<Vec<u8>>,
    ) -> Result<MemoryTier, String>
```

## Visibility

- `pub`

## Docstring

Evict/offload a buffer from GPU VRAM into pinned DDR5 host memory.
In actual execution, this copies device memory over PCIe Gen4/Gen5 into aligned host pages.

## Source
Lines 88–134 in `crates/model-trainer/src/ddr5_offload.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ddr5_offload](/crates/model-trainer/src/ddr5_offload.md) |
