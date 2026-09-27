---
okf_version: "0.2"
type: Function
title: remove_peer
description: Remove a disconnected peer
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/remove_peer_1
language: rust
---

# remove_peer

Remove a disconnected peer

## Signature

```rust
pub fn remove_peer(&self, node_id: &NodeId, reason: &str)
```

## Visibility

- `pub`

## Docstring

Remove a disconnected peer

## Source
Lines 289–298 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
