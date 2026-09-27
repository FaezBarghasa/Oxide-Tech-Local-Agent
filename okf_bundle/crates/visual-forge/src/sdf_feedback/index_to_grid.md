---
okf_version: "0.2"
type: Function
title: index_to_grid
description: "Converts a 1D flat index into 3D grid indices (gx, gy, gz)."
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/index_to_grid
language: rust
---

# index_to_grid

Converts a 1D flat index into 3D grid indices (gx, gy, gz).

## Signature

```rust
impl SDFVolume { pub fn index_to_grid(&self, idx: usize) -> [usize; 3] }
```

## Visibility

- `pub`

## Docstring

Converts a 1D flat index into 3D grid indices (gx, gy, gz).
[inline]

## Source
Lines 100–102 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
