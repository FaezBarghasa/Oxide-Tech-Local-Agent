---
okf_version: "0.2"
type: Class
title: CircuitState
description: "Circuit electrical state for SMT-LIB2 invariant translation:"
resource: crates/formal-verify/src/smt_solver.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T16:28:38Z"
concept_id: crates/formal-verify/src/smt_solver/CircuitState
language: rust
---

# CircuitState

Circuit electrical state for SMT-LIB2 invariant translation:

## Signature

```rust
pub struct CircuitState
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Circuit electrical state for SMT-LIB2 invariant translation:
Phi_safe = bigwedge_{s in States} (V(s) <= V_max /\ I(s) <= I_max /\ (Fault(s) ==> Isolated(s)))
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `state_id`
- `voltage`
- `max_voltage`
- `current`
- `max_current`
- `has_fault`
- `is_isolated`

## Source
Lines 68–76 in `crates/formal-verify/src/smt_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smt_solver](/crates/formal-verify/src/smt_solver.md) |
