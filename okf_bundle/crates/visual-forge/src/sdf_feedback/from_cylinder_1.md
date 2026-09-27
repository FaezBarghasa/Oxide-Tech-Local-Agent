---
okf_version: "0.2"
type: Function
title: from_cylinder
description: Rasterizes an analytical Capped Cylinder (aligned along Z-axis) into an SDF volume.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/from_cylinder_1
language: rust
---

# from_cylinder

Rasterizes an analytical Capped Cylinder (aligned along Z-axis) into an SDF volume.

## Signature

```rust
pub fn from_cylinder(
        center: Vec3,
        radius: f32,
        height: f32,
        bounds: [Vec3; 2],
        resolution: f32,
    ) -> Self
```

## Visibility

- `pub`

## Docstring

Rasterizes an analytical Capped Cylinder (aligned along Z-axis) into an SDF volume.

## Source
Lines 181–213 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
