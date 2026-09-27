---
okf_version: "0.2"
type: Class
title: Mutation
description: A specific mutation applied to a CAD operation node in the Feature DAG.
resource: crates/parametric-forge/src/patcher.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/patcher/Mutation
language: rust
---

# Mutation

A specific mutation applied to a CAD operation node in the Feature DAG.

## Signature

```rust
pub enum Mutation
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "mutation_type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

A specific mutation applied to a CAD operation node in the Feature DAG.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "mutation_type", rename_all = "snake_case")]

## Methods

- `point_idx`
- `position`

## Source
Lines 24–36 in `crates/parametric-forge/src/patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [patcher](/crates/parametric-forge/src/patcher.md) |
