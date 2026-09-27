---
okf_version: "0.2"
type: Function
title: add_peer
description: Register a connected peer
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/add_peer
language: rust
---

# add_peer

Register a connected peer

## Signature

```rust
impl QuicMeshTransport { pub fn add_peer(&self, node_id: NodeId, addr: SocketAddr) }
```

## Visibility

- `pub`

## Docstring

Register a connected peer

## Source
Lines 276–286 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
