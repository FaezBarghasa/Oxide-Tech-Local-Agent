---
okf_version: "0.2"
type: Function
title: forward
description: Forward pass through pooled latent state
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/forward
language: rust
---

# forward

Forward pass through pooled latent state

## Signature

```rust
impl FastKanDecisionHead { pub fn forward(&self, pooled_state: &[f32]) -> Vec<f32> }
```

## Visibility

- `pub`

## Docstring

Forward pass through pooled latent state

## Source
Lines 286–303 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
