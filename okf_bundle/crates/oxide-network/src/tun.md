---
okf_version: "0.2"
type: Module
title: tun
description: "# Virtual Network Interface Engine (TUN) & MSS Clamping (`crates/oxide-network/src/tun.rs`)"
resource: crates/oxide-network/src/tun.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:36Z"
concept_id: crates/oxide-network/src/tun
language: rust
---

# tun

# Virtual Network Interface Engine (TUN) & MSS Clamping (`crates/oxide-network/src/tun.rs`)

## Docstring

# Virtual Network Interface Engine (TUN) & MSS Clamping (`crates/oxide-network/src/tun.rs`)

Provides virtual network device abstractions and dynamic TCP MSS clamping
to prevent packet fragmentation over encapsulated overlay tunnels.

## Relationships

| Type | Target |
|------|--------|
| related | [TunConfig](/crates/oxide-network/src/tun/TunConfig.md) |
| related | [default](/crates/oxide-network/src/tun/default.md) |
| related | [default](/crates/oxide-network/src/tun/default.md) |
| related | [clamp_tcp_mss](/crates/oxide-network/src/tun/clamp_tcp_mss.md) |
| related | [recompute_tcp_checksum](/crates/oxide-network/src/tun/recompute_tcp_checksum.md) |
| related | [TunDevice](/crates/oxide-network/src/tun/TunDevice.md) |
| related | [new](/crates/oxide-network/src/tun/new.md) |
| related | [config](/crates/oxide-network/src/tun/config.md) |
| related | [is_active](/crates/oxide-network/src/tun/is_active.md) |
| related | [new](/crates/oxide-network/src/tun/new.md) |
| related | [config](/crates/oxide-network/src/tun/config.md) |
| related | [is_active](/crates/oxide-network/src/tun/is_active.md) |
| related | [test_tcp_mss_clamping](/crates/oxide-network/src/tun/test_tcp_mss_clamping.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
