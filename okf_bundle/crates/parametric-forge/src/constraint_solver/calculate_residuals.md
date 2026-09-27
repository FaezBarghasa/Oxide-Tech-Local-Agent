---
okf_version: "0.2"
type: Function
title: calculate_residuals
description: Evaluates constraint equations $F(x) = 0$.
resource: crates/parametric-forge/src/constraint_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/constraint_solver/calculate_residuals
language: rust
---

# calculate_residuals

Evaluates constraint equations $F(x) = 0$.

## Signature

```rust
impl ConstraintSolver { fn calculate_residuals(
        constraints: &[GeometricConstraint],
        x: &[f64],
        num_points: usize,
    ) -> Vec<f64> }
```

## Docstring

Evaluates constraint equations $F(x) = 0$.

## Source
Lines 137–202 in `crates/parametric-forge/src/constraint_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint_solver](/crates/parametric-forge/src/constraint_solver.md) |
