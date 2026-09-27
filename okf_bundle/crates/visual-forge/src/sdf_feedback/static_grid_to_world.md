---
okf_version: "0.2"
type: Function
title: static_grid_to_world
description: Converts 3D grid coordinates to world-space coordinates (voxel center) statically.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/static_grid_to_world
language: rust
---

# static_grid_to_world

Converts 3D grid coordinates to world-space coordinates (voxel center) statically.

## Signature

```rust
impl SDFVolume { pub fn static_grid_to_world(
        gx: usize,
        gy: usize,
        gz: usize,
        bounds: [Vec3; 2],
        resolution: f32,
    ) -> Vec3 }
```

## Visibility

- `pub`

## Docstring

Converts 3D grid coordinates to world-space coordinates (voxel center) statically.
[inline]

## Source
Lines 84–96 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
