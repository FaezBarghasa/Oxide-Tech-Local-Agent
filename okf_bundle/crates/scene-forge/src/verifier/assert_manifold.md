---
okf_version: "0.2"
type: Function
title: assert_manifold
description: "Verifies 2-manifoldness and watertightness using edge adjacency and the Euler-Poincaré formula:"
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/assert_manifold
language: rust
---

# assert_manifold

Verifies 2-manifoldness and watertightness using edge adjacency and the Euler-Poincaré formula:

## Signature

```rust
impl GeometryVerifier { pub fn assert_manifold(&self, mesh: &MeshData) -> ManifoldReport }
```

## Visibility

- `pub`

## Docstring

Verifies 2-manifoldness and watertightness using edge adjacency and the Euler-Poincaré formula:
V - E + F = 2 (for genus-0 closed polyhedra)

## Source
Lines 49–100 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
