---
okf_version: "0.2"
type: Function
title: all_gather_parameter
description: Simulate All-Gather across the Ring Topology to reconstruct full tensor for forward/backward pass
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/all_gather_parameter
language: rust
---

# all_gather_parameter

Simulate All-Gather across the Ring Topology to reconstruct full tensor for forward/backward pass

## Signature

```rust
impl DistributedEngine { pub fn all_gather_parameter(&self, name: &str) -> Option<Vec<f32>> }
```

## Visibility

- `pub`

## Docstring

Simulate All-Gather across the Ring Topology to reconstruct full tensor for forward/backward pass

## Source
Lines 157–181 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
