---
okf_version: "0.2"
type: Function
title: send_datagram
description: Send an encrypted WirePacket datagram to target node
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/send_datagram
language: rust
---

# send_datagram

Send an encrypted WirePacket datagram to target node

## Signature

```rust
impl QuicMeshTransport { pub fn send_datagram(&self, node_id: NodeId, packet: WirePacket) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Send an encrypted WirePacket datagram to target node

## Source
Lines 247–273 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
