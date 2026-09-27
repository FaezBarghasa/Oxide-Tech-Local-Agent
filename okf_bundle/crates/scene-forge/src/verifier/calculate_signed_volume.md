---
okf_version: "0.2"
type: Function
title: calculate_signed_volume
description: "Calculate the exact signed volume of a closed triangle mesh using the Divergence Theorem:"
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/calculate_signed_volume
language: rust
---

# calculate_signed_volume

Calculate the exact signed volume of a closed triangle mesh using the Divergence Theorem:

## Signature

```rust
impl GeometryVerifier { pub fn calculate_signed_volume(&self, mesh: &MeshData) -> f32 }
```

## Visibility

- `pub`

## Docstring

Calculate the exact signed volume of a closed triangle mesh using the Divergence Theorem:
Volume = 1/6 * sum( v0 . (v1 x v2) )

## Source
Lines 104–123 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
