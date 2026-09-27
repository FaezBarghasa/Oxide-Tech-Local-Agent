---
okf_version: "0.2"
type: Function
title: compute_normals
description: Recompute flat/smooth vertex normals from face geometry.
resource: crates/scene-forge/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/mesh/compute_normals
language: rust
---

# compute_normals

Recompute flat/smooth vertex normals from face geometry.

## Signature

```rust
impl MeshData { pub fn compute_normals(&mut self) }
```

## Visibility

- `pub`

## Docstring

Recompute flat/smooth vertex normals from face geometry.

## Source
Lines 30–61 in `crates/scene-forge/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/scene-forge/src/mesh.md) |
