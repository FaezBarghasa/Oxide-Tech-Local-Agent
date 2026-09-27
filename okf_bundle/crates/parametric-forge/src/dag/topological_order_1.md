---
okf_version: "0.2"
type: Function
title: topological_order
description: Return topological ordering of operations from root features to leaves.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/topological_order_1
language: rust
---

# topological_order

Return topological ordering of operations from root features to leaves.

## Signature

```rust
pub fn topological_order(&self) -> Result<Vec<NodeIndex>, ParametricError>
```

## Visibility

- `pub`

## Docstring

Return topological ordering of operations from root features to leaves.

## Source
Lines 202–209 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
