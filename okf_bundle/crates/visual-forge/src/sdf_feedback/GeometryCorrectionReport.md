---
okf_version: "0.2"
type: Class
title: GeometryCorrectionReport
description: Detailed geometric correction report produced by deterministic 3D volumetric diffing.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/GeometryCorrectionReport
language: rust
---

# GeometryCorrectionReport

Detailed geometric correction report produced by deterministic 3D volumetric diffing.

## Signature

```rust
pub struct GeometryCorrectionReport
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Detailed geometric correction report produced by deterministic 3D volumetric diffing.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `missing_material_mm3`
- `excess_material_mm3`
- `max_surface_deviation_mm`
- `deviation_center`
- `volumetric_iou`
- `correction_hint`

## Source
Lines 7–20 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
