---
okf_version: "0.2"
type: Function
title: add_component
description: Add a component to the circuit graph with default auto footprint.
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/add_component_1
language: rust
---

# add_component

Add a component to the circuit graph with default auto footprint.

## Signature

```rust
pub fn add_component(&mut self, ref_des: &str, lib_id: &str, value: &str) -> &mut Self
```

## Visibility

- `pub`

## Docstring

Add a component to the circuit graph with default auto footprint.

## Source
Lines 62–64 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
