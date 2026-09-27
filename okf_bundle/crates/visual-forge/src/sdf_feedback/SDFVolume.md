---
okf_version: "0.2"
type: Class
title: SDFVolume
description: A 3D Volumetric Signed Distance Field (SDF) grid.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/SDFVolume
language: rust
---

# SDFVolume

A 3D Volumetric Signed Distance Field (SDF) grid.

## Signature

```rust
pub struct SDFVolume
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A 3D Volumetric Signed Distance Field (SDF) grid.

Distances:
- `< 0.0`: Inside the solid boundary
- `== 0.0`: Exactly on the surface
- `> 0.0`: Outside the solid boundary
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `voxels`
- `resolution`
- `dim`
- `bounds`

## Source
Lines 29–34 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
