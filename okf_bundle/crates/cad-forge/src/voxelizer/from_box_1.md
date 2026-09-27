---
okf_version: "0.2"
type: Function
title: from_box
description: Voxelize a box volume in parallel using Rayon
resource: crates/cad-forge/src/voxelizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/cad-forge/src/voxelizer/from_box_1
language: rust
---

# from_box

Voxelize a box volume in parallel using Rayon

## Signature

```rust
pub fn from_box(center: Vec3, size: Vec3, resolution: f32) -> Self
```

## Visibility

- `pub`

## Docstring

Voxelize a box volume in parallel using Rayon

## Source
Lines 56–59 in `crates/cad-forge/src/voxelizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [voxelizer](/crates/cad-forge/src/voxelizer.md) |
