---
okf_version: "0.2"
type: Function
title: from_json
description: "Parse a JSON string representing `CircuitScript` or a list of `CircuitCommand`s and build the graph."
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/from_json
language: rust
---

# from_json

Parse a JSON string representing `CircuitScript` or a list of `CircuitCommand`s and build the graph.

## Signature

```rust
impl CircuitBuilder { pub fn from_json(json_str: &str) -> Result<CircuitGraph, CircuitError> }
```

## Visibility

- `pub`

## Docstring

Parse a JSON string representing `CircuitScript` or a list of `CircuitCommand`s and build the graph.

## Source
Lines 191–201 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
