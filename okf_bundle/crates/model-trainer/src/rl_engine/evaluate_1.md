---
okf_version: "0.2"
type: Function
title: evaluate
description: "Compute composite scalar reward:"
resource: crates/model-trainer/src/rl_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:47:41Z"
concept_id: crates/model-trainer/src/rl_engine/evaluate_1
language: rust
---

# evaluate

Compute composite scalar reward:

## Signature

```rust
pub fn evaluate(
        &self,
        r_compile: f32,
        r_formal_verify: f32,
        r_drc_pass: f32,
        r_sim_continuity: f32,
    ) -> f32
```

## Visibility

- `pub`

## Docstring

Compute composite scalar reward:
Reward(o_i) = w1 * R_compile + w2 * R_formal_verify + w3 * R_drc_pass + w4 * R_sim_continuity

## Source
Lines 183–194 in `crates/model-trainer/src/rl_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rl_engine](/crates/model-trainer/src/rl_engine.md) |
