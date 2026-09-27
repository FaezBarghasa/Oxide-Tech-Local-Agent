---
okf_version: "0.2"
type: Function
title: solve_linear_system
description: "Solves linear system A * x = b via Gaussian elimination with partial pivoting."
resource: crates/parametric-forge/src/constraint_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/constraint_solver/solve_linear_system_1
language: rust
---

# solve_linear_system

Solves linear system A * x = b via Gaussian elimination with partial pivoting.

## Signature

```rust
fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>>
```

## Decorators

- `allow(clippy::needless_range_loop)`

## Docstring

Solves linear system A * x = b via Gaussian elimination with partial pivoting.
[allow(clippy::needless_range_loop)]

## Source
Lines 237–285 in `crates/parametric-forge/src/constraint_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint_solver](/crates/parametric-forge/src/constraint_solver.md) |
