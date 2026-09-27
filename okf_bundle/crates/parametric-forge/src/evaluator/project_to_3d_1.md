---
okf_version: "0.2"
type: Function
title: project_to_3d
description: Project 2D sketch points onto the specified 3D plane.
resource: crates/parametric-forge/src/evaluator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/evaluator/project_to_3d_1
language: rust
---

# project_to_3d

Project 2D sketch points onto the specified 3D plane.

## Signature

```rust
fn project_to_3d(points: &[[f64; 2]], plane: Plane) -> Vec<[f64; 3]>
```

## Docstring

Project 2D sketch points onto the specified 3D plane.

## Source
Lines 139–149 in `crates/parametric-forge/src/evaluator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evaluator](/crates/parametric-forge/src/evaluator.md) |
