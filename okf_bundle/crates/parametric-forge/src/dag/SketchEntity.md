---
okf_version: "0.2"
type: Class
title: SketchEntity
description: 2D geometric entity inside a sketch.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/SketchEntity
language: rust
---

# SketchEntity

2D geometric entity inside a sketch.

## Signature

```rust
pub enum SketchEntity
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "entity_type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

2D geometric entity inside a sketch.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "entity_type", rename_all = "snake_case")]

## Methods

- `id`
- `x`
- `y`
- `p1`
- `p2`
- `center`
- `radius`
- `start_angle`
- `end_angle`

## Source
Lines 36–52 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
