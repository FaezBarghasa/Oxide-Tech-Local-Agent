---
okf_version: "0.2"
type: Function
title: new
description: Create a new pin connection with a pin name.
resource: crates/circuit-forge/src/netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:17:21Z"
concept_id: crates/circuit-forge/src/netlist/new
language: rust
---

# new

Create a new pin connection with a pin name.

## Signature

```rust
impl PinConnection { pub fn new(pin_name: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new pin connection with a pin name.

## Source
Lines 100–105 in `crates/circuit-forge/src/netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/circuit-forge/src/netlist.md) |
