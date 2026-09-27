---
okf_version: "0.2"
type: Class
title: DeltaPatch
description: A fine-grained delta patch that modifies only a masked hierarchy level without regenerating full history.
resource: crates/parametric-forge/src/patcher.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/patcher/DeltaPatch
language: rust
---

# DeltaPatch

A fine-grained delta patch that modifies only a masked hierarchy level without regenerating full history.

## Signature

```rust
pub struct DeltaPatch
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A fine-grained delta patch that modifies only a masked hierarchy level without regenerating full history.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `target_node`
- `hierarchy_level`
- `mutation`

## Source
Lines 40–44 in `crates/parametric-forge/src/patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [patcher](/crates/parametric-forge/src/patcher.md) |
