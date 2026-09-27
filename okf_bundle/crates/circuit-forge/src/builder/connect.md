---
okf_version: "0.2"
type: Function
title: connect
description: "Connect two components' pins via a named net."
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/connect
language: rust
---

# connect

Connect two components' pins via a named net.

## Signature

```rust
impl CircuitBuilder { pub fn connect(
        &mut self,
        comp1: &str,
        pin1: &str,
        comp2: &str,
        pin2: &str,
        net_name: &str,
    ) -> Result<&mut Self, CircuitError> }
```

## Visibility

- `pub`

## Docstring

Connect two components' pins via a named net.

## Source
Lines 99–127 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
