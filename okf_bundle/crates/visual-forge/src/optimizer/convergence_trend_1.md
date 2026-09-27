---
okf_version: "0.2"
type: Function
title: convergence_trend
description: Computes convergence progress across iterations.
resource: crates/visual-forge/src/optimizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:00:45Z"
concept_id: crates/visual-forge/src/optimizer/convergence_trend_1
language: rust
---

# convergence_trend

Computes convergence progress across iterations.

## Signature

```rust
pub fn convergence_trend(&self) -> (f64, bool)
```

## Visibility

- `pub`

## Docstring

Computes convergence progress across iterations.
Returns `(current_iou, is_improving)`.

## Source
Lines 44–54 in `crates/visual-forge/src/optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [optimizer](/crates/visual-forge/src/optimizer.md) |
