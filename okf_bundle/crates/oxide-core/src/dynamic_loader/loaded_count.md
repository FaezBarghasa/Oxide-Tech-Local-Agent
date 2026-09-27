---
okf_version: "0.2"
type: Function
title: loaded_count
description: Total active loaded dynamic libraries.
resource: crates/oxide-core/src/dynamic_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/dynamic_loader/loaded_count
language: rust
---

# loaded_count

Total active loaded dynamic libraries.

## Signature

```rust
impl DynamicSkillLoader { pub fn loaded_count(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Total active loaded dynamic libraries.

## Source
Lines 91–94 in `crates/oxide-core/src/dynamic_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dynamic_loader](/crates/oxide-core/src/dynamic_loader.md) |
