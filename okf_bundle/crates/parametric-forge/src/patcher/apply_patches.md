---
okf_version: "0.2"
type: Function
title: apply_patches
description: "Applies a batch of `DeltaPatch` commands in sequence."
resource: crates/parametric-forge/src/patcher.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/patcher/apply_patches
language: rust
---

# apply_patches

Applies a batch of `DeltaPatch` commands in sequence.

## Signature

```rust
impl FeatureDAG { pub fn apply_patches(&mut self, patches: &[DeltaPatch]) -> Result<(), ParametricError> }
```

## Visibility

- `pub`

## Docstring

Applies a batch of `DeltaPatch` commands in sequence.

## Source
Lines 179–184 in `crates/parametric-forge/src/patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [patcher](/crates/parametric-forge/src/patcher.md) |
