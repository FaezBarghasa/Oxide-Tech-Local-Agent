---
okf_version: "0.2"
type: Function
title: from_extruded_polygon
description: Rasterizes an extruded 2D convex/simple polygon into an SDF volume.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/from_extruded_polygon
language: rust
---

# from_extruded_polygon

Rasterizes an extruded 2D convex/simple polygon into an SDF volume.

## Signature

```rust
impl SDFVolume { pub fn from_extruded_polygon(
        polygon: &[[f32; 2]],
        z_min: f32,
        z_max: f32,
        bounds: [Vec3; 2],
        resolution: f32,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

Rasterizes an extruded 2D convex/simple polygon into an SDF volume.

## Source
Lines 216–249 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
