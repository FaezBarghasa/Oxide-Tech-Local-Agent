---
okf_version: "0.2"
type: Function
title: world_to_grid
description: Converts a world-space point to integer grid coordinates if within bounds.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/world_to_grid_1
language: rust
---

# world_to_grid

Converts a world-space point to integer grid coordinates if within bounds.

## Signature

```rust
pub fn world_to_grid(&self, p: Vec3) -> Option<[usize; 3]>
```

## Visibility

- `pub`

## Docstring

Converts a world-space point to integer grid coordinates if within bounds.

## Source
Lines 118–138 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
