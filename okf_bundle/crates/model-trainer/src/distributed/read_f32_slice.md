---
okf_version: "0.2"
type: Function
title: read_f32_slice
description: Extract zero-copy f32 slice directly from mapped memory
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/read_f32_slice
language: rust
---

# read_f32_slice

Extract zero-copy f32 slice directly from mapped memory

## Signature

```rust
impl MmapGgufWeightLoader { pub fn read_f32_slice(&self, tensor_name: &str) -> Option<&[f32]> }
```

## Visibility

- `pub`

## Docstring

Extract zero-copy f32 slice directly from mapped memory

## Source
Lines 79–95 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
