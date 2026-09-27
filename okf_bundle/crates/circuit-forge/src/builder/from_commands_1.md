---
okf_version: "0.2"
type: Function
title: from_commands
description: "Execute a sequence of `CircuitCommand`s to construct a `CircuitGraph`."
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/from_commands_1
language: rust
---

# from_commands

Execute a sequence of `CircuitCommand`s to construct a `CircuitGraph`.

## Signature

```rust
pub fn from_commands(commands: &[CircuitCommand]) -> Result<CircuitGraph, CircuitError>
```

## Visibility

- `pub`

## Docstring

Execute a sequence of `CircuitCommand`s to construct a `CircuitGraph`.

## Source
Lines 156–188 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
