---
okf_version: "0.2"
type: Function
title: compute_volumetric_diff
description: Computes the deterministic 3D volumetric difference between generated CAD and target CAD.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/compute_volumetric_diff_1
language: rust
---

# compute_volumetric_diff

Computes the deterministic 3D volumetric difference between generated CAD and target CAD.

## Signature

```rust
pub fn compute_volumetric_diff(&self, target: &SDFVolume) -> GeometryCorrectionReport
```

## Visibility

- `pub`

## Docstring

Computes the deterministic 3D volumetric difference between generated CAD and target CAD.

Returns a structured `GeometryCorrectionReport` that can be directly parsed
by LLM agents without running 2D rendering or VLM visual encoders.

## Source
Lines 345–471 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
