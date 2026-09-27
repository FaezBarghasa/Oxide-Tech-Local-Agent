---
okf_version: "0.2"
type: Function
title: insert
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/insert
language: rust
---

# insert

## Signature

```rust
impl CandidateVectorCache { pub fn insert(&mut self, candidate_id: impl Into<String>, vector: Vec<f32>) }
```

## Visibility

- `pub`

## Source
Lines 185–190 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
