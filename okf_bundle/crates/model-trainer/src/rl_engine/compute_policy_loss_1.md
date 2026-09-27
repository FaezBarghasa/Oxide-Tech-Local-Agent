---
okf_version: "0.2"
type: Function
title: compute_policy_loss
description: Compute GRPO surrogate loss for a token sequence with PPO clipping and reference KL penalty
resource: crates/model-trainer/src/rl_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:47:41Z"
concept_id: crates/model-trainer/src/rl_engine/compute_policy_loss_1
language: rust
---

# compute_policy_loss

Compute GRPO surrogate loss for a token sequence with PPO clipping and reference KL penalty

## Signature

```rust
pub fn compute_policy_loss(
        &self,
        pi_logp: &[f32],
        old_logp: &[f32],
        ref_logp: &[f32],
        advantage: f32,
    ) -> f32
```

## Visibility

- `pub`

## Docstring

Compute GRPO surrogate loss for a token sequence with PPO clipping and reference KL penalty

## Source
Lines 131–157 in `crates/model-trainer/src/rl_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rl_engine](/crates/model-trainer/src/rl_engine.md) |
