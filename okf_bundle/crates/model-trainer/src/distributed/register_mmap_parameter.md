---
okf_version: "0.2"
type: Function
title: register_mmap_parameter
description: Register partitioned parameter directly from memory-mapped GGUF loader
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/register_mmap_parameter
language: rust
---

# register_mmap_parameter

Register partitioned parameter directly from memory-mapped GGUF loader

## Signature

```rust
impl DistributedEngine { pub fn register_mmap_parameter(
        &self,
        name: &str,
        mmap_loader: &MmapGgufWeightLoader,
        tensor_name: &str,
    ) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Register partitioned parameter directly from memory-mapped GGUF loader

## Source
Lines 143–154 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
