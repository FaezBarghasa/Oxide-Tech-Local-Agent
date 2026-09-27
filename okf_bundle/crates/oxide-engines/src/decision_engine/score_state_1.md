---
okf_version: "0.2"
type: Function
title: score_state
description: Dense dot-product similarity lookup against state embedding
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/score_state_1
language: rust
---

# score_state

Dense dot-product similarity lookup against state embedding

## Signature

```rust
pub fn score_state(&self, state_embedding: &[f32]) -> Option<(String, f32)>
```

## Visibility

- `pub`

## Docstring

Dense dot-product similarity lookup against state embedding

## Source
Lines 193–196 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
