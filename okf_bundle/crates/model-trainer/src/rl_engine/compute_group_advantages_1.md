---
okf_version: "0.2"
type: Function
title: compute_group_advantages
description: "Compute normalized group advantages: A_i = (R_i - mean(R)) / (std(R) + eps)"
resource: crates/model-trainer/src/rl_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:47:41Z"
concept_id: crates/model-trainer/src/rl_engine/compute_group_advantages_1
language: rust
---

# compute_group_advantages

Compute normalized group advantages: A_i = (R_i - mean(R)) / (std(R) + eps)

## Signature

```rust
pub fn compute_group_advantages(&self, rewards: &[f32]) -> Vec<f32>
```

## Visibility

- `pub`

## Docstring

Compute normalized group advantages: A_i = (R_i - mean(R)) / (std(R) + eps)

## Source
Lines 117–128 in `crates/model-trainer/src/rl_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rl_engine](/crates/model-trainer/src/rl_engine.md) |
