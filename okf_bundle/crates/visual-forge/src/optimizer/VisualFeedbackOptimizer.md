---
okf_version: "0.2"
type: Class
title: VisualFeedbackOptimizer
description: Closed-loop visual feedback optimizer guiding parametric adjustments based on volumetric error deltas.
resource: crates/visual-forge/src/optimizer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:00:45Z"
concept_id: crates/visual-forge/src/optimizer/VisualFeedbackOptimizer
language: rust
---

# VisualFeedbackOptimizer

Closed-loop visual feedback optimizer guiding parametric adjustments based on volumetric error deltas.

## Signature

```rust
pub struct VisualFeedbackOptimizer
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Closed-loop visual feedback optimizer guiding parametric adjustments based on volumetric error deltas.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `target_sdf`
- `tolerance_iou`
- `max_allowed_deviation_mm`
- `history`

## Source
Lines 7–12 in `crates/visual-forge/src/optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [optimizer](/crates/visual-forge/src/optimizer.md) |
