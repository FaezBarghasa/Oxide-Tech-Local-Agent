---
okf_version: "0.2"
type: Function
title: apply_patch
description: "Applies a hierarchy-aware `DeltaPatch` directly to the target node in the Feature DAG."
resource: crates/parametric-forge/src/patcher.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/patcher/apply_patch_1
language: rust
---

# apply_patch

Applies a hierarchy-aware `DeltaPatch` directly to the target node in the Feature DAG.

## Signature

```rust
pub fn apply_patch(&mut self, patch: &DeltaPatch) -> Result<(), ParametricError>
```

## Visibility

- `pub`

## Docstring

Applies a hierarchy-aware `DeltaPatch` directly to the target node in the Feature DAG.

## Source
Lines 58–176 in `crates/parametric-forge/src/patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [patcher](/crates/parametric-forge/src/patcher.md) |
