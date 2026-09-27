---
okf_version: "0.2"
type: Function
title: reduce_scatter_gradients
description: Ring Reduce-Scatter gradients across cluster ranks
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/reduce_scatter_gradients
language: rust
---

# reduce_scatter_gradients

Ring Reduce-Scatter gradients across cluster ranks

## Signature

```rust
impl DistributedEngine { pub fn reduce_scatter_gradients(&self, _name: &str, global_grads: &[f32]) -> Vec<f32> }
```

## Visibility

- `pub`

## Docstring

Ring Reduce-Scatter gradients across cluster ranks

## Source
Lines 184–194 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
