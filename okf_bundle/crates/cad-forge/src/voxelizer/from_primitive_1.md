---
okf_version: "0.2"
type: Function
title: from_primitive
description: Voxelize an individual primitive with Rayon parallel scanning
resource: crates/cad-forge/src/voxelizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/cad-forge/src/voxelizer/from_primitive_1
language: rust
---

# from_primitive

Voxelize an individual primitive with Rayon parallel scanning

## Signature

```rust
pub fn from_primitive(primitive: &Primitive, resolution: f32) -> Self
```

## Visibility

- `pub`

## Docstring

Voxelize an individual primitive with Rayon parallel scanning

## Source
Lines 62–116 in `crates/cad-forge/src/voxelizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [voxelizer](/crates/cad-forge/src/voxelizer.md) |
