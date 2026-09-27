---
okf_version: "0.2"
type: Class
title: NetlistNode
description: Represents a node in the bipartite circuit netlist graph.
resource: crates/circuit-forge/src/netlist.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:17:21Z"
concept_id: crates/circuit-forge/src/netlist/NetlistNode
language: rust
---

# NetlistNode

Represents a node in the bipartite circuit netlist graph.

## Signature

```rust
pub enum NetlistNode
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`
- `serde(tag = "type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Represents a node in the bipartite circuit netlist graph.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
[serde(tag = "type", rename_all = "snake_case")]

## Methods

- `lib_id`
- `ref_des`
- `value`
- `footprint`
- `name`
- `id`

## Source
Lines 8–27 in `crates/circuit-forge/src/netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/circuit-forge/src/netlist.md) |
