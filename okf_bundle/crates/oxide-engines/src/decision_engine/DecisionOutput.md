---
okf_version: "0.2"
type: Class
title: DecisionOutput
description: "[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]"
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/DecisionOutput
language: rust
---

# DecisionOutput

[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Signature

```rust
pub struct DecisionOutput
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `selected`
- `confidence`
- `confidence_spread`
- `escalated`
- `latency`
- `candidate_scores`

## Source
Lines 13–20 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
