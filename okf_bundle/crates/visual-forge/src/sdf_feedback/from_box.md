---
okf_version: "0.2"
type: Function
title: from_box
description: Rasterizes an analytical axis-aligned Box primitive into an SDF volume.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/from_box
language: rust
---

# from_box

Rasterizes an analytical axis-aligned Box primitive into an SDF volume.

## Signature

```rust
impl SDFVolume { pub fn from_box(center: Vec3, size: Vec3, bounds: [Vec3; 2], resolution: f32) -> Self }
```

## Visibility

- `pub`

## Docstring

Rasterizes an analytical axis-aligned Box primitive into an SDF volume.

## Source
Lines 141–160 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
