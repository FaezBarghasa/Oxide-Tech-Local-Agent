---
okf_version: "0.2"
type: Function
title: new
description: Creates a new Shadow Kernel with specified evaluation bounding box and voxel resolution.
resource: crates/visual-forge/src/shadow_kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/shadow_kernel/new
language: rust
---

# new

Creates a new Shadow Kernel with specified evaluation bounding box and voxel resolution.

## Signature

```rust
impl ShadowKernel { pub fn new(min_bound: [f32; 3], max_bound: [f32; 3], resolution: f32) -> Self }
```

## Visibility

- `pub`

## Docstring

Creates a new Shadow Kernel with specified evaluation bounding box and voxel resolution.

## Source
Lines 60–68 in `crates/visual-forge/src/shadow_kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_kernel](/crates/visual-forge/src/shadow_kernel.md) |
