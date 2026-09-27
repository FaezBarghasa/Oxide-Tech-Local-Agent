---
okf_version: "0.2"
type: Class
title: DecisionEngine
description: Thread-safe client handle. Can be cloned across OS threads or Rayon tasks.
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/DecisionEngine
language: rust
---

# DecisionEngine

Thread-safe client handle. Can be cloned across OS threads or Rayon tasks.

## Signature

```rust
pub struct DecisionEngine
```

## Decorators

- `derive(Clone)`

## Visibility

- `pub`

## Docstring

Thread-safe client handle. Can be cloned across OS threads or Rayon tasks.
[derive(Clone)]

## Methods

- `tx`
- `default_cascade`

## Source
Lines 56–59 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
