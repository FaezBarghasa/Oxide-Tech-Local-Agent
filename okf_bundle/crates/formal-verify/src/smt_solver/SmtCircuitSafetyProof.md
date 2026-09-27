---
okf_version: "0.2"
type: Class
title: SmtCircuitSafetyProof
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/formal-verify/src/smt_solver.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T16:28:38Z"
concept_id: crates/formal-verify/src/smt_solver/SmtCircuitSafetyProof
language: rust
---

# SmtCircuitSafetyProof

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct SmtCircuitSafetyProof
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `is_safe`
- `smt_lib2_formula`
- `violated_states`

## Source
Lines 79–83 in `crates/formal-verify/src/smt_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smt_solver](/crates/formal-verify/src/smt_solver.md) |
