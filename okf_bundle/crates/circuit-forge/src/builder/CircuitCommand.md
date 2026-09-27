---
okf_version: "0.2"
type: Class
title: CircuitCommand
description: Structured commands for programmatic or LLM-driven circuit creation.
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/CircuitCommand
language: rust
---

# CircuitCommand

Structured commands for programmatic or LLM-driven circuit creation.

## Signature

```rust
pub enum CircuitCommand
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`
- `serde(tag = "action", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Structured commands for programmatic or LLM-driven circuit creation.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
[serde(tag = "action", rename_all = "snake_case")]

## Methods

- `ref_des`
- `lib_id`
- `value`
- `footprint`
- `comp1`
- `pin1`
- `comp2`
- `pin2`
- `net_name`
- `comp`
- `pin`
- `net_name`

## Source
Lines 12–35 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
