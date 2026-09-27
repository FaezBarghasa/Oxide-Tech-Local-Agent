---
okf_version: "0.2"
type: Function
title: grid_to_index
description: "Converts 3D grid indices (gx, gy, gz) into a 1D flat index."
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/grid_to_index_1
language: rust
---

# grid_to_index

Converts 3D grid indices (gx, gy, gz) into a 1D flat index.

## Signature

```rust
pub fn grid_to_index(&self, gx: usize, gy: usize, gz: usize) -> usize
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Converts 3D grid indices (gx, gy, gz) into a 1D flat index.
[inline]

## Source
Lines 67–69 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
