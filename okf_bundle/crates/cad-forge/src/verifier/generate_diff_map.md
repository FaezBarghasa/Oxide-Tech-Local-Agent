---
okf_version: "0.2"
type: Function
title: generate_diff_map
description: Generates a comprehensive 3D difference map for closed-loop VLM agent self-correction.
resource: crates/cad-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cad-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:10:28Z"
concept_id: crates/cad-forge/src/verifier/generate_diff_map
language: rust
---

# generate_diff_map

Generates a comprehensive 3D difference map for closed-loop VLM agent self-correction.

## Signature

```rust
pub fn generate_diff_map(generated: &VoxelGrid, target: &VoxelGrid) -> VoxelDiffMap
```

## Visibility

- `pub`

## Docstring

Generates a comprehensive 3D difference map for closed-loop VLM agent self-correction.

## Source
Lines 44–68 in `crates/cad-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/cad-forge/src/verifier.md) |
| called_by | [test_csg_subtraction_voxelization](/crates/cad-forge/tests/cad_tests/test_csg_subtraction_voxelization.md) |
