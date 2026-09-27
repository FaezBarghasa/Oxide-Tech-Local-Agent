---
okf_version: "0.2"
type: Function
title: is_peer_allowed
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/is_peer_allowed
language: rust
---

# is_peer_allowed

## Signature

```rust
impl TransportCircuitBreaker { pub fn is_peer_allowed(&self, node_id: &NodeId) -> bool }
```

## Visibility

- `pub`

## Source
Lines 193–202 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
