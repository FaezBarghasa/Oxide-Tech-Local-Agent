---
okf_version: "0.2"
type: Function
title: static_index_to_grid
description: "Converts a 1D flat index into 3D grid indices (gx, gy, gz) statically without borrowing self."
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/static_index_to_grid_1
language: rust
---

# static_index_to_grid

Converts a 1D flat index into 3D grid indices (gx, gy, gz) statically without borrowing self.

## Signature

```rust
pub fn static_index_to_grid(idx: usize, dim: [usize; 3]) -> [usize; 3]
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Converts a 1D flat index into 3D grid indices (gx, gy, gz) statically without borrowing self.
[inline]

## Source
Lines 73–80 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
