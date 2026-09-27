---
okf_version: "0.2"
type: Function
title: is_point_inside
description: Check if a world point lies inside a primitive
resource: crates/cad-forge/src/voxelizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/cad-forge/src/voxelizer/is_point_inside_1
language: rust
---

# is_point_inside

Check if a world point lies inside a primitive

## Signature

```rust
fn is_point_inside(primitive: &Primitive, pt: Vec3) -> bool
```

## Decorators

- `inline(always)`

## Docstring

Check if a world point lies inside a primitive
[inline(always)]

## Source
Lines 23–53 in `crates/cad-forge/src/voxelizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [voxelizer](/crates/cad-forge/src/voxelizer.md) |
