---
okf_version: "0.2"
type: Class
title: MeshData
description: "High-performance 3D mesh representation compatible with binary IPC (`postcard`)."
resource: crates/scene-forge/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/mesh/MeshData
language: rust
---

# MeshData

High-performance 3D mesh representation compatible with binary IPC (`postcard`).

## Signature

```rust
pub struct MeshData
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

High-performance 3D mesh representation compatible with binary IPC (`postcard`).
[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `vertices`
- `normals`
- `indices`

## Source
Lines 6–10 in `crates/scene-forge/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/scene-forge/src/mesh.md) |
