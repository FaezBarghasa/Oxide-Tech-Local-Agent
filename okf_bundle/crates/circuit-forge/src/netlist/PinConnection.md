---
okf_version: "0.2"
type: Class
title: PinConnection
description: Represents an edge in the netlist bipartite graph connecting a Component to a Net.
resource: crates/circuit-forge/src/netlist.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:17:21Z"
concept_id: crates/circuit-forge/src/netlist/PinConnection
language: rust
---

# PinConnection

Represents an edge in the netlist bipartite graph connecting a Component to a Net.

## Signature

```rust
pub struct PinConnection
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Represents an edge in the netlist bipartite graph connecting a Component to a Net.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `pin_name`
- `pin_number`

## Source
Lines 90–96 in `crates/circuit-forge/src/netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/circuit-forge/src/netlist.md) |
