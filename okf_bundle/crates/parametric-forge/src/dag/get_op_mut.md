---
okf_version: "0.2"
type: Function
title: get_op_mut
description: Get mutable operation reference by UUID.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/get_op_mut
language: rust
---

# get_op_mut

Get mutable operation reference by UUID.

## Signature

```rust
impl FeatureDAG { pub fn get_op_mut(&mut self, id: Uuid) -> Option<&mut CadOperation> }
```

## Visibility

- `pub`

## Docstring

Get mutable operation reference by UUID.

## Source
Lines 193–199 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
