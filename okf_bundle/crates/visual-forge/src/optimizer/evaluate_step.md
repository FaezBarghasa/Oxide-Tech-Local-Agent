---
okf_version: "0.2"
type: Function
title: evaluate_step
description: "Evaluates the current generated solid state against the target, recording step telemetry."
resource: crates/visual-forge/src/optimizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:00:45Z"
concept_id: crates/visual-forge/src/optimizer/evaluate_step
language: rust
---

# evaluate_step

Evaluates the current generated solid state against the target, recording step telemetry.

## Signature

```rust
impl VisualFeedbackOptimizer { pub fn evaluate_step(&mut self, current_sdf: &SDFVolume) -> GeometryCorrectionReport }
```

## Visibility

- `pub`

## Docstring

Evaluates the current generated solid state against the target, recording step telemetry.

## Source
Lines 26–30 in `crates/visual-forge/src/optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [optimizer](/crates/visual-forge/src/optimizer.md) |
