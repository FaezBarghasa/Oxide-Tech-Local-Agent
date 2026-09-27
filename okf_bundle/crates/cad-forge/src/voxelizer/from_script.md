---
okf_version: "0.2"
type: Function
title: from_script
description: Voxelize a full CAD script with CSG boolean operations
resource: crates/cad-forge/src/voxelizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/cad-forge/src/voxelizer/from_script
language: rust
---

# from_script

Voxelize a full CAD script with CSG boolean operations

## Signature

```rust
impl VoxelGrid { pub fn from_script(script: &CadScript, resolution: f32) -> Self }
```

## Visibility

- `pub`

## Docstring

Voxelize a full CAD script with CSG boolean operations

## Source
Lines 119–162 in `crates/cad-forge/src/voxelizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [voxelizer](/crates/cad-forge/src/voxelizer.md) |
