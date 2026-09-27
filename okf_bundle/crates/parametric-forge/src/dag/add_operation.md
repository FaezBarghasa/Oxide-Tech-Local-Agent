---
okf_version: "0.2"
type: Function
title: add_operation
description: Add a CAD operation to the DAG and automatically create dependency edges if referenced IDs exist.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/add_operation
language: rust
---

# add_operation

Add a CAD operation to the DAG and automatically create dependency edges if referenced IDs exist.

## Signature

```rust
impl FeatureDAG { pub fn add_operation(&mut self, op: CadOperation) -> NodeIndex }
```

## Visibility

- `pub`

## Docstring

Add a CAD operation to the DAG and automatically create dependency edges if referenced IDs exist.

## Source
Lines 141–164 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
