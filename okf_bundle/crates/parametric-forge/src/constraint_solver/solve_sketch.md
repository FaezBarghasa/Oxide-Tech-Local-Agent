---
okf_version: "0.2"
type: Function
title: solve_sketch
description: Solves the 2D sketch point coordinates deterministically to satisfy all geometric constraints.
resource: crates/parametric-forge/src/constraint_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/constraint_solver/solve_sketch
language: rust
---

# solve_sketch

Solves the 2D sketch point coordinates deterministically to satisfy all geometric constraints.

## Signature

```rust
impl ConstraintSolver { pub fn solve_sketch(
        constraints: &[GeometricConstraint],
        points: &mut [[f64; 2]],
        max_iterations: usize,
        tolerance: f64,
    ) -> Result<(), ParametricError> }
```

## Visibility

- `pub`

## Docstring

Solves the 2D sketch point coordinates deterministically to satisfy all geometric constraints.

## Source
Lines 37–134 in `crates/parametric-forge/src/constraint_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint_solver](/crates/parametric-forge/src/constraint_solver.md) |
