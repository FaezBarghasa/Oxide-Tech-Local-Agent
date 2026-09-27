---
okf_version: "0.2"
type: Function
title: calculate_jacobian
description: Computes the Jacobian matrix using central finite differences.
resource: crates/parametric-forge/src/constraint_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/constraint_solver/calculate_jacobian_1
language: rust
---

# calculate_jacobian

Computes the Jacobian matrix using central finite differences.

## Signature

```rust
fn calculate_jacobian(
        constraints: &[GeometricConstraint],
        x: &[f64],
        num_points: usize,
    ) -> Vec<Vec<f64>>
```

## Docstring

Computes the Jacobian matrix using central finite differences.

## Source
Lines 205–233 in `crates/parametric-forge/src/constraint_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint_solver](/crates/parametric-forge/src/constraint_solver.md) |
