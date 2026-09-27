---
okf_version: "0.2"
type: Function
title: process_inbound_datagram
description: Process an inbound encapsulated datagram from a remote mesh peer
resource: crates/oxide-network/src/remote_access.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:54Z"
concept_id: crates/oxide-network/src/remote_access/process_inbound_datagram
language: rust
---

# process_inbound_datagram

Process an inbound encapsulated datagram from a remote mesh peer

## Signature

```rust
impl RemoteAccessMeshEngine { pub fn process_inbound_datagram(
        &self,
        from_node: NodeId,
        packet_bytes: &[u8],
    ) -> Result<Vec<u8>, OxideError> }
```

## Visibility

- `pub`

## Docstring

Process an inbound encapsulated datagram from a remote mesh peer

## Source
Lines 258–305 in `crates/oxide-network/src/remote_access.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_access](/crates/oxide-network/src/remote_access.md) |
| calls | [pre_parse_packet](/crates/oxide-network/src/wire/pre_parse_packet.md) |
