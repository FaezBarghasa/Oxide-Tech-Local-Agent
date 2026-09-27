---
okf_version: "0.2"
type: Function
title: cube
description: "Generates a watertight 8-vertex, 12-triangle cube centered at the origin."
resource: crates/scene-forge/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/mesh/cube
language: rust
---

# cube

Generates a watertight 8-vertex, 12-triangle cube centered at the origin.

## Signature

```rust
pub fn cube(dimensions: [f32; 3]) -> MeshData
```

## Visibility

- `pub`

## Docstring

Generates a watertight 8-vertex, 12-triangle cube centered at the origin.

## Source
Lines 69–110 in `crates/scene-forge/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/scene-forge/src/mesh.md) |
| called_by | [handle_command](/crates/scene-forge/src/ipc_bridge/handle_command.md) |
| called_by | [sci_fi_crate](/crates/scene-forge/src/mesh/sci_fi_crate.md) |
| called_by | [test_manifold_verification_watertight_cube](/crates/scene-forge/tests/scene_tests/test_manifold_verification_watertight_cube.md) |
| called_by | [test_non_manifold_detection_missing_face](/crates/scene-forge/tests/scene_tests/test_non_manifold_detection_missing_face.md) |
| called_by | [test_signed_volume_calculation](/crates/scene-forge/tests/scene_tests/test_signed_volume_calculation.md) |
