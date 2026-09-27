---
okf_version: "0.2"
type: Function
title: new
description: Creates a new closed-loop optimizer targeting the specified reference SDF volume.
resource: crates/visual-forge/src/optimizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:00:45Z"
concept_id: crates/visual-forge/src/optimizer/new
language: rust
---

# new

Creates a new closed-loop optimizer targeting the specified reference SDF volume.

## Signature

```rust
impl VisualFeedbackOptimizer { pub fn new(target_sdf: SDFVolume, tolerance_iou: f64, max_allowed_deviation_mm: f32) -> Self }
```

## Visibility

- `pub`

## Docstring

Creates a new closed-loop optimizer targeting the specified reference SDF volume.

## Source
Lines 16–23 in `crates/visual-forge/src/optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [optimizer](/crates/visual-forge/src/optimizer.md) |
