---
okf_version: "0.2"
type: Function
title: find_node_by_uuid
description: Find node index by UUID.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/find_node_by_uuid
language: rust
---

# find_node_by_uuid

Find node index by UUID.

## Signature

```rust
impl FeatureDAG { pub fn find_node_by_uuid(&self, id: Uuid) -> Result<NodeIndex, ParametricError> }
```

## Visibility

- `pub`

## Docstring

Find node index by UUID.

## Source
Lines 180–185 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
