---
okf_version: "0.2"
type: Function
title: solve
description: Solve 2D geometric constraints using iterative relaxation (Newton-Raphson step)
resource: crates/cad-forge/src/sketch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/cad-forge/src/sketch/solve
language: rust
---

# solve

Solve 2D geometric constraints using iterative relaxation (Newton-Raphson step)

## Signature

```rust
impl SketchConstraintSolver { pub fn solve(&self, sketch: &mut Sketch, constraints: &[Constraint2D]) -> bool }
```

## Visibility

- `pub`

## Docstring

Solve 2D geometric constraints using iterative relaxation (Newton-Raphson step)

## Source
Lines 41–111 in `crates/cad-forge/src/sketch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch](/crates/cad-forge/src/sketch.md) |
