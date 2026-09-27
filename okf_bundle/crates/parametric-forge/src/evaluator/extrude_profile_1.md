---
okf_version: "0.2"
type: Function
title: extrude_profile
description: Extrude a 3D closed polygon profile along a direction vector by distance.
resource: crates/parametric-forge/src/evaluator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/evaluator/extrude_profile_1
language: rust
---

# extrude_profile

Extrude a 3D closed polygon profile along a direction vector by distance.

## Signature

```rust
fn extrude_profile(
        id: Uuid,
        name: &str,
        profile: &[[f64; 3]],
        distance: f64,
        direction: [f64; 3],
    ) -> Result<EvaluatedSolid, ParametricError>
```

## Docstring

Extrude a 3D closed polygon profile along a direction vector by distance.

## Source
Lines 152–229 in `crates/parametric-forge/src/evaluator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evaluator](/crates/parametric-forge/src/evaluator.md) |
