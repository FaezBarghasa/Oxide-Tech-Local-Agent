---
okf_version: "0.2"
type: Class
title: TransportEvent
description: Transport event notifications emitted across the mesh
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/TransportEvent
language: rust
---

# TransportEvent

Transport event notifications emitted across the mesh

## Signature

```rust
pub enum TransportEvent
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Transport event notifications emitted across the mesh
[derive(Debug, Clone)]

## Methods

- `node_id`
- `endpoint`
- `node_id`
- `reason`
- `from`
- `packet`
- `node_id`
- `new_endpoint`

## Source
Lines 18–33 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
