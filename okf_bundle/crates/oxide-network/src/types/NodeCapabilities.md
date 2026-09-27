---
okf_version: "0.2"
type: Class
title: NodeCapabilities
description: Node capabilities advertised in the mesh
resource: crates/oxide-network/src/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:05:47Z"
concept_id: crates/oxide-network/src/types/NodeCapabilities
language: rust
---

# NodeCapabilities

Node capabilities advertised in the mesh

## Signature

```rust
pub struct NodeCapabilities
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Node capabilities advertised in the mesh
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Methods

- `subnet_router`
- `exit_node`
- `dns_server`
- `file_transfer`
- `ssh_server`
- `http_proxy`
- `relay`

## Source
Lines 279–287 in `crates/oxide-network/src/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-network/src/types.md) |
