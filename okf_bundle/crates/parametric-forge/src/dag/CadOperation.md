---
okf_version: "0.2"
type: Class
title: CadOperation
description: Parametric CAD construction history operation.
resource: crates/parametric-forge/src/dag.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/dag/CadOperation
language: rust
---

# CadOperation

Parametric CAD construction history operation.

## Signature

```rust
pub enum CadOperation
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "op_type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Parametric CAD construction history operation.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "op_type", rename_all = "snake_case")]

## Methods

- `id`
- `name`
- `plane`
- `points`
- `constraints`
- `entities`
- `id`
- `name`
- `profile_id`
- `distance`
- `direction`
- `id`
- `name`
- `target_op_id`
- `target_edges`
- `radius`
- `id`
- `name`
- `target_op_id`
- `target_edges`
- `distance`
- `id`
- `name`
- `boolean_op`
- `target_a`
- `target_b`

## Source
Lines 57–94 in `crates/parametric-forge/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/parametric-forge/src/dag.md) |
