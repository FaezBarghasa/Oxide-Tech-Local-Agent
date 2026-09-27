---
okf_version: "0.2"
type: Function
title: new_empty
description: Creates a new empty SDF volume with all voxels set to infinity (outside).
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/new_empty_1
language: rust
---

# new_empty

Creates a new empty SDF volume with all voxels set to infinity (outside).

## Signature

```rust
pub fn new_empty(bounds: [Vec3; 2], resolution: f32) -> Self
```

## Visibility

- `pub`

## Docstring

Creates a new empty SDF volume with all voxels set to infinity (outside).

## Source
Lines 38–51 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
