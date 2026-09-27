---
okf_version: "0.2"
type: Class
title: Plane
description: Coordinate reference plane for 2D sketches.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/Plane
language: rust
---

# Plane

Coordinate reference plane for 2D sketches.

## Signature

```rust
pub enum Plane
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)`
- `serde(tag = "plane_type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Coordinate reference plane for 2D sketches.
[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
[serde(tag = "plane_type", rename_all = "snake_case")]

## Methods

- `origin`
- `normal`

## Source
Lines 13–22 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
