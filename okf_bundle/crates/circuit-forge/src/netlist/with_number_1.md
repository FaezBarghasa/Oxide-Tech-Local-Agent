---
okf_version: "0.2"
type: Function
title: with_number
description: Create a new pin connection with both pin name and physical pin number.
resource: crates/circuit-forge/src/netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:17:21Z"
concept_id: crates/circuit-forge/src/netlist/with_number_1
language: rust
---

# with_number

Create a new pin connection with both pin name and physical pin number.

## Signature

```rust
pub fn with_number(pin_name: impl Into<String>, pin_number: impl Into<String>) -> Self
```

## Visibility

- `pub`

## Docstring

Create a new pin connection with both pin name and physical pin number.

## Source
Lines 108–113 in `crates/circuit-forge/src/netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/circuit-forge/src/netlist.md) |
