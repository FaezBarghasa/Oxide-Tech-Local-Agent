---
okf_version: "0.2"
type: Function
title: get_op
description: Get operation reference by UUID.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/get_op
language: rust
---

# get_op

Get operation reference by UUID.

## Signature

```rust
impl FeatureDAG { pub fn get_op(&self, id: Uuid) -> Option<&CadOperation> }
```

## Visibility

- `pub`

## Docstring

Get operation reference by UUID.

## Source
Lines 188–190 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
