---
okf_version: "0.2"
type: Function
title: add_dependency
description: Add a dependency edge from parent operation to child operation.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/add_dependency
language: rust
---

# add_dependency

Add a dependency edge from parent operation to child operation.

## Signature

```rust
impl FeatureDAG { pub fn add_dependency(
        &mut self,
        from_id: Uuid,
        to_id: Uuid,
        edge: DependencyEdge,
    ) -> Result<(), ParametricError> }
```

## Visibility

- `pub`

## Docstring

Add a dependency edge from parent operation to child operation.

## Source
Lines 167–177 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
