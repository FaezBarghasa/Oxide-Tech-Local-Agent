---
okf_version: "0.2"
type: Function
title: score_candidates_ranked
description: Ranks all candidate options with calibrated probabilities
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/score_candidates_ranked
language: rust
---

# score_candidates_ranked

Ranks all candidate options with calibrated probabilities

## Signature

```rust
impl CandidateVectorCache { pub fn score_candidates_ranked(&self, state_embedding: &[f32]) -> Vec<(String, f32)> }
```

## Visibility

- `pub`

## Docstring

Ranks all candidate options with calibrated probabilities

## Source
Lines 199–222 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
