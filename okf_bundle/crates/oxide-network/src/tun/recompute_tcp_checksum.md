---
okf_version: "0.2"
type: Function
title: recompute_tcp_checksum
description: Helper function to recompute checksum
resource: crates/oxide-network/src/tun.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:36Z"
concept_id: crates/oxide-network/src/tun/recompute_tcp_checksum
language: rust
---

# recompute_tcp_checksum

Helper function to recompute checksum

## Signature

```rust
fn recompute_tcp_checksum(packet: &mut [u8], ip_header_len: usize, version: u8)
```

## Docstring

Helper function to recompute checksum

## Source
Lines 115–152 in `crates/oxide-network/src/tun.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tun](/crates/oxide-network/src/tun.md) |
| called_by | [clamp_tcp_mss](/crates/oxide-network/src/tun/clamp_tcp_mss.md) |
