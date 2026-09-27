---
okf_version: "0.2"
type: Function
title: lib_id
description: Returns the KiCad lib_id if this node is a component.
resource: crates/circuit-forge/src/netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:17:21Z"
concept_id: crates/circuit-forge/src/netlist/lib_id
language: rust
---

# lib_id

Returns the KiCad lib_id if this node is a component.

## Signature

```rust
impl NetlistNode { pub fn lib_id(&self) -> Option<&str> }
```

## Visibility

- `pub`

## Docstring

Returns the KiCad lib_id if this node is a component.
[inline]

## Source
Lines 71–76 in `crates/circuit-forge/src/netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/circuit-forge/src/netlist.md) |
