---
okf_version: "0.2"
type: Function
title: cylinder
description: Generates a watertight cylinder with top and bottom caps.
resource: crates/scene-forge/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/mesh/cylinder
language: rust
---

# cylinder

Generates a watertight cylinder with top and bottom caps.

## Signature

```rust
pub fn cylinder(radius: f32, height: f32, segments: u32) -> MeshData
```

## Visibility

- `pub`

## Docstring

Generates a watertight cylinder with top and bottom caps.

## Source
Lines 113–162 in `crates/scene-forge/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/scene-forge/src/mesh.md) |
| called_by | [handle_command](/crates/scene-forge/src/ipc_bridge/handle_command.md) |
