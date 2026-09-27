---
okf_version: "0.2"
type: Function
title: connect_net
description: Connect a component pin to a named net.
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/connect_net
language: rust
---

# connect_net

Connect a component pin to a named net.

## Signature

```rust
impl CircuitBuilder { pub fn connect_net(
        &mut self,
        comp: &str,
        pin: &str,
        net_name: &str,
    ) -> Result<&mut Self, CircuitError> }
```

## Visibility

- `pub`

## Docstring

Connect a component pin to a named net.

## Source
Lines 130–148 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
