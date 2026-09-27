---
okf_version: "0.2"
type: Function
title: register_tensor_offset
description: Register tensor byte offset within memory-mapped buffer
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/register_tensor_offset
language: rust
---

# register_tensor_offset

Register tensor byte offset within memory-mapped buffer

## Signature

```rust
impl MmapGgufWeightLoader { pub fn register_tensor_offset(&mut self, tensor_name: impl Into<String>, offset: usize, count: usize) }
```

## Visibility

- `pub`

## Docstring

Register tensor byte offset within memory-mapped buffer

## Source
Lines 74–76 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
