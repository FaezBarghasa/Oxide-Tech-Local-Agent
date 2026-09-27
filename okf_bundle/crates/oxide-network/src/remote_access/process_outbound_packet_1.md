---
okf_version: "0.2"
type: Function
title: process_outbound_packet
description: Process an outbound IP packet from the local TUN interface
resource: crates/oxide-network/src/remote_access.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:54Z"
concept_id: crates/oxide-network/src/remote_access/process_outbound_packet_1
language: rust
---

# process_outbound_packet

Process an outbound IP packet from the local TUN interface

## Signature

```rust
pub fn process_outbound_packet(
        &self,
        packet: &mut [u8],
        l1_cache: &mut L1DirectMappedCache,
    ) -> Result<Option<NodeId>, OxideError>
```

## Visibility

- `pub`

## Docstring

Process an outbound IP packet from the local TUN interface

## Source
Lines 198–255 in `crates/oxide-network/src/remote_access.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_access](/crates/oxide-network/src/remote_access.md) |
| calls | [clamp_tcp_mss](/crates/oxide-network/src/tun/clamp_tcp_mss.md) |
