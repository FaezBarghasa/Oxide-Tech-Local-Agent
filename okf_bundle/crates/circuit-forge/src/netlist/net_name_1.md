---
okf_version: "0.2"
type: Function
title: net_name
description: Returns the net name if this node is a net.
resource: crates/circuit-forge/src/netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:17:21Z"
concept_id: crates/circuit-forge/src/netlist/net_name_1
language: rust
---

# net_name

Returns the net name if this node is a net.

## Signature

```rust
pub fn net_name(&self) -> Option<&str>
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Returns the net name if this node is a net.
[inline]

## Source
Lines 53–58 in `crates/circuit-forge/src/netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/circuit-forge/src/netlist.md) |
