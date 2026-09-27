---
okf_version: "0.2"
type: Function
title: forward
description: Compute ORPO loss fusing Cross Entropy with Odds-Ratio penalty
resource: crates/model-trainer/src/rl_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:47:41Z"
concept_id: crates/model-trainer/src/rl_engine/forward_3
language: rust
---

# forward

Compute ORPO loss fusing Cross Entropy with Odds-Ratio penalty

## Signature

```rust
pub fn forward(&self, sft_nll_loss: f32, pi_chosen_logp: f32, pi_rejected_logp: f32) -> f32
```

## Visibility

- `pub`

## Docstring

Compute ORPO loss fusing Cross Entropy with Odds-Ratio penalty

## Source
Lines 78–87 in `crates/model-trainer/src/rl_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rl_engine](/crates/model-trainer/src/rl_engine.md) |
