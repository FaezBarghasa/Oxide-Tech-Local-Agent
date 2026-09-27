---
okf_version: "0.2"
type: Function
title: register_parameter
description: Partition a global parameter across world_size ranks according to ZeRO stage
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/register_parameter
language: rust
---

# register_parameter

Partition a global parameter across world_size ranks according to ZeRO stage

## Signature

```rust
impl DistributedEngine { pub fn register_parameter(&self, name: &str, weights: &[f32]) }
```

## Visibility

- `pub`

## Docstring

Partition a global parameter across world_size ranks according to ZeRO stage

## Source
Lines 117–140 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
