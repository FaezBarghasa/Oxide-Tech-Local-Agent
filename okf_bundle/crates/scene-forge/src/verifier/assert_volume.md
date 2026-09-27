---
okf_version: "0.2"
type: Function
title: assert_volume
description: Assert that the calculated volume matches the expected volume within a tolerance.
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/assert_volume
language: rust
---

# assert_volume

Assert that the calculated volume matches the expected volume within a tolerance.

## Signature

```rust
impl GeometryVerifier { pub fn assert_volume(&self, mesh: &MeshData, expected_volume: f32, tolerance: f32) -> bool }
```

## Visibility

- `pub`

## Docstring

Assert that the calculated volume matches the expected volume within a tolerance.

## Source
Lines 126–129 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
