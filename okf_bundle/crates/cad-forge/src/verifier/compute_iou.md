---
okf_version: "0.2"
type: Function
title: compute_iou
description: Rayon-accelerated multi-core 3D IoU calculation between two sparse voxel grids.
resource: crates/cad-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:10:28Z"
concept_id: crates/cad-forge/src/verifier/compute_iou
language: rust
---

# compute_iou

Rayon-accelerated multi-core 3D IoU calculation between two sparse voxel grids.

## Signature

```rust
pub fn compute_iou(generated: &VoxelGrid, target: &VoxelGrid) -> f64
```

## Visibility

- `pub`

## Docstring

Rayon-accelerated multi-core 3D IoU calculation between two sparse voxel grids.

## Source
Lines 14–41 in `crates/cad-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/cad-forge/src/verifier.md) |
| called_by | [test_voxel_grid_disjoint_boxes_iou](/crates/cad-forge/tests/cad_tests/test_voxel_grid_disjoint_boxes_iou.md) |
| called_by | [test_voxel_grid_identical_box_iou](/crates/cad-forge/tests/cad_tests/test_voxel_grid_identical_box_iou.md) |
