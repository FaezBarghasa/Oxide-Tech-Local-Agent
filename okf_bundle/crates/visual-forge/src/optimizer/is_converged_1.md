---
okf_version: "0.2"
type: Function
title: is_converged
description: Checks whether the model has converged to within the target IoU and surface deviation tolerances.
resource: crates/visual-forge/src/optimizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:00:45Z"
concept_id: crates/visual-forge/src/optimizer/is_converged_1
language: rust
---

# is_converged

Checks whether the model has converged to within the target IoU and surface deviation tolerances.

## Signature

```rust
pub fn is_converged(&self) -> bool
```

## Visibility

- `pub`

## Docstring

Checks whether the model has converged to within the target IoU and surface deviation tolerances.

## Source
Lines 33–40 in `crates/visual-forge/src/optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [optimizer](/crates/visual-forge/src/optimizer.md) |
