---
okf_version: "0.2"
type: Function
title: get_or_create_net
description: Get existing net node index or create a new one.
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/get_or_create_net
language: rust
---

# get_or_create_net

Get existing net node index or create a new one.

## Signature

```rust
impl CircuitBuilder { pub fn get_or_create_net(&mut self, net_name: &str) -> NodeIndex }
```

## Visibility

- `pub`

## Docstring

Get existing net node index or create a new one.

## Source
Lines 85–96 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
