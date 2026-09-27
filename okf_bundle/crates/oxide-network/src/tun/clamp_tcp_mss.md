---
okf_version: "0.2"
type: Function
title: clamp_tcp_mss
description: In-place dynamic TCP MSS (Maximum Segment Size) Clamping
resource: crates/oxide-network/src/tun.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:36Z"
concept_id: crates/oxide-network/src/tun/clamp_tcp_mss
language: rust
---

# clamp_tcp_mss

In-place dynamic TCP MSS (Maximum Segment Size) Clamping

## Signature

```rust
pub fn clamp_tcp_mss(packet: &mut [u8], max_mss: u16) -> Result<bool, OxideError>
```

## Visibility

- `pub`

## Docstring

In-place dynamic TCP MSS (Maximum Segment Size) Clamping
Modifies SYN / SYN-ACK packets to enforce MSS <= max_mss

## Source
Lines 34–112 in `crates/oxide-network/src/tun.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tun](/crates/oxide-network/src/tun.md) |
| calls | [recompute_tcp_checksum](/crates/oxide-network/src/tun/recompute_tcp_checksum.md) |
| called_by | [process_outbound_packet](/crates/oxide-network/src/remote_access/process_outbound_packet.md) |
| called_by | [test_tcp_mss_clamping](/crates/oxide-network/src/tun/test_tcp_mss_clamping.md) |
