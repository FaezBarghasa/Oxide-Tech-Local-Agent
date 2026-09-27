---
okf_version: "0.2"
type: Function
title: sci_fi_crate
description: Generates a low-poly sci-fi crate mesh with parametric dimensions.
resource: crates/scene-forge/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/mesh/sci_fi_crate
language: rust
---

# sci_fi_crate

Generates a low-poly sci-fi crate mesh with parametric dimensions.

## Signature

```rust
pub fn sci_fi_crate(dimensions: [f32; 3], _bevel_offset: f32) -> MeshData
```

## Visibility

- `pub`

## Docstring

Generates a low-poly sci-fi crate mesh with parametric dimensions.

## Source
Lines 165–168 in `crates/scene-forge/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/scene-forge/src/mesh.md) |
| calls | [cube](/crates/scene-forge/src/mesh/cube.md) |
| called_by | [handle_command](/crates/scene-forge/src/ipc_bridge/handle_command.md) |
