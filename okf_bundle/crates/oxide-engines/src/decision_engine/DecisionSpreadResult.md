---
okf_version: "0.2"
type: Class
title: DecisionSpreadResult
description: Structured candidate ranking and speculative spread evaluation result
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/DecisionSpreadResult
language: rust
---

# DecisionSpreadResult

Structured candidate ranking and speculative spread evaluation result

## Signature

```rust
pub struct DecisionSpreadResult
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Structured candidate ranking and speculative spread evaluation result
[derive(Debug, Clone, PartialEq)]

## Methods

- `selected`
- `confidence`
- `confidence_spread`
- `escalated`
- `candidate_scores`

## Source
Lines 172–178 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
