---
okf_version: "0.2"
type: Function
title: calculate_bounding_box
description: "Calculate the axis-aligned bounding box (AABB) (min, max) of the mesh."
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/calculate_bounding_box_1
language: rust
---

# calculate_bounding_box

Calculate the axis-aligned bounding box (AABB) (min, max) of the mesh.

## Signature

```rust
pub fn calculate_bounding_box(&self, mesh: &MeshData) -> ([f32; 3], [f32; 3])
```

## Visibility

- `pub`

## Docstring

Calculate the axis-aligned bounding box (AABB) (min, max) of the mesh.

## Source
Lines 132–147 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
