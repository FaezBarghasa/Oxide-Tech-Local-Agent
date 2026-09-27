---
okf_version: "0.2"
type: Function
title: sd_polygon_2d
description: Computes exact signed distance from point p to 2D polygon.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/sd_polygon_2d_1
language: rust
---

# sd_polygon_2d

Computes exact signed distance from point p to 2D polygon.

## Signature

```rust
fn sd_polygon_2d(p: Vec2, poly: &[[f32; 2]]) -> f32
```

## Docstring

Computes exact signed distance from point p to 2D polygon.

## Source
Lines 252–286 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
