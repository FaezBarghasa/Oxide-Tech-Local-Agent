---
okf_version: "0.2"
type: Function
title: verify_circuit_safety_invariants
description: Translate circuit state machine into SMT-LIB2 assertions and solve safety invariants
resource: crates/formal-verify/src/smt_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T16:28:38Z"
concept_id: crates/formal-verify/src/smt_solver/verify_circuit_safety_invariants
language: rust
---

# verify_circuit_safety_invariants

Translate circuit state machine into SMT-LIB2 assertions and solve safety invariants

## Signature

```rust
pub fn verify_circuit_safety_invariants(states: &[CircuitState]) -> SmtCircuitSafetyProof
```

## Visibility

- `pub`

## Docstring

Translate circuit state machine into SMT-LIB2 assertions and solve safety invariants

## Source
Lines 86–120 in `crates/formal-verify/src/smt_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smt_solver](/crates/formal-verify/src/smt_solver.md) |
