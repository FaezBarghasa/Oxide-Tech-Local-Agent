---
okf_version: "0.2"
type: Function
title: compute
description: "Compute Brier score: 1/N * sum((predicted_prob - actual_binary_outcome)^2)"
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/compute
language: rust
---

# compute

Compute Brier score: 1/N * sum((predicted_prob - actual_binary_outcome)^2)

## Signature

```rust
impl BrierScoreLoss { pub fn compute(predictions: &[f32], targets: &[f32]) -> f32 }
```

## Visibility

- `pub`

## Docstring

Compute Brier score: 1/N * sum((predicted_prob - actual_binary_outcome)^2)

## Source
Lines 257–267 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
