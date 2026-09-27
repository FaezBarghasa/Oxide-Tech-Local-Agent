---
okf_version: "0.2"
type: Function
title: forward
description: Compute DPO loss from policy and reference log probabilities
resource: crates/model-trainer/src/rl_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:47:41Z"
concept_id: crates/model-trainer/src/rl_engine/forward_1
language: rust
---

# forward

Compute DPO loss from policy and reference log probabilities

## Signature

```rust
pub fn forward(
        &self,
        pi_chosen_logp: f32,
        pi_rejected_logp: f32,
        ref_chosen_logp: f32,
        ref_rejected_logp: f32,
    ) -> (f32, f32, f32)
```

## Visibility

- `pub`

## Docstring

Compute DPO loss from policy and reference log probabilities
Loss = -log(sigmoid(beta * (log(pi_theta(yw)/ref(yw)) - log(pi_theta(yl)/ref(yl)))))

## Source
Lines 36–58 in `crates/model-trainer/src/rl_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rl_engine](/crates/model-trainer/src/rl_engine.md) |
