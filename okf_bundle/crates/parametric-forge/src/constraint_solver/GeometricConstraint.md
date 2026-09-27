---
okf_version: "0.2"
type: Class
title: GeometricConstraint
description: Geometric 2D constraint applied to sketch control points.
resource: crates/parametric-forge/src/constraint_solver.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/constraint_solver/GeometricConstraint
language: rust
---

# GeometricConstraint

Geometric 2D constraint applied to sketch control points.

## Signature

```rust
pub enum GeometricConstraint
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "constraint_type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Geometric 2D constraint applied to sketch control points.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "constraint_type", rename_all = "snake_case")]

## Methods

- `point`
- `x`
- `y`
- `p1`
- `p2`
- `p1`
- `p2`
- `p1`
- `p2`
- `distance`
- `p1`
- `p2`
- `point`
- `line_p1`
- `line_p2`

## Source
Lines 8–25 in `crates/parametric-forge/src/constraint_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint_solver](/crates/parametric-forge/src/constraint_solver.md) |
