---
okf_version: "0.2"
type: Class
title: ManifoldReport
description: Report from mathematical topological verification of 3D geometry.
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/ManifoldReport
language: rust
---

# ManifoldReport

Report from mathematical topological verification of 3D geometry.

## Signature

```rust
pub struct ManifoldReport
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Report from mathematical topological verification of 3D geometry.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `is_manifold`
- `is_closed`
- `euler_characteristic`
- `vertex_count`
- `edge_count`
- `face_count`
- `boundary_edges`
- `non_manifold_edges`

## Source
Lines 9–18 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
