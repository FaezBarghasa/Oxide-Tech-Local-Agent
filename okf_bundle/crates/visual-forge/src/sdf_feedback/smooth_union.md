---
okf_version: "0.2"
type: Function
title: smooth_union
description: Polynomial Smooth Boolean Union (fillet-like blending).
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/smooth_union
language: rust
---

# smooth_union

Polynomial Smooth Boolean Union (fillet-like blending).

## Signature

```rust
impl SDFVolume { pub fn smooth_union(&self, other: &SDFVolume, k: f32) -> Self }
```

## Visibility

- `pub`

## Docstring

Polynomial Smooth Boolean Union (fillet-like blending).

## Source
Lines 328–339 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
