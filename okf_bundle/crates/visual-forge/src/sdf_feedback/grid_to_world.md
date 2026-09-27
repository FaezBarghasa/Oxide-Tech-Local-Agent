---
okf_version: "0.2"
type: Function
title: grid_to_world
description: Converts 3D grid coordinates to world-space coordinates (voxel center).
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/grid_to_world
language: rust
---

# grid_to_world

Converts 3D grid coordinates to world-space coordinates (voxel center).

## Signature

```rust
impl SDFVolume { pub fn grid_to_world(&self, gx: usize, gy: usize, gz: usize) -> Vec3 }
```

## Visibility

- `pub`

## Docstring

Converts 3D grid coordinates to world-space coordinates (voxel center).
[inline]

## Source
Lines 106–108 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
