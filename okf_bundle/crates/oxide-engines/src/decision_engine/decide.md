---
okf_version: "0.2"
type: Function
title: decide
description: "Synchronous, blocking call. Thread-safe and re-entrant."
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/decide
language: rust
---

# decide

Synchronous, blocking call. Thread-safe and re-entrant.

## Signature

```rust
impl DecisionEngine { pub fn decide(&self, input: DecisionInput) -> Result<DecisionOutput, String> }
```

## Visibility

- `pub`

## Docstring

Synchronous, blocking call. Thread-safe and re-entrant.

## Source
Lines 95–97 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
