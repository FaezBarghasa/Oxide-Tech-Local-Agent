---
okf_version: "0.2"
type: Module
title: lib
description: "# Zero-Trust Remote Access & Mesh Network Subsystem (`crates/oxide-network`)"
resource: crates/oxide-network/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:07:35Z"
concept_id: crates/oxide-network/src/lib
language: rust
---

# lib

# Zero-Trust Remote Access & Mesh Network Subsystem (`crates/oxide-network`)

## Docstring

# Zero-Trust Remote Access & Mesh Network Subsystem (`crates/oxide-network`)

Master remote overlay networking stack implementing:
- Pure-Rust multi-queue TUN interface abstraction and dynamic TCP MSS clamping
- 128-bit sliding bitmask anti-replay protection and anti-DPI junk frame classification
- Lock-free Read-Copy-Update (RCU) Radix Longest-Prefix-Match routing with L1 cache
- Ed25519 identity, Blake3 KDF, and authenticated AEAD encryption/decryption
- Quinn QUIC datagram (RFC 9221) transport with PortHopper RSS striping & DPLPMTUD
- Zero-Trust Access Control Lists (ACL) microsegmentation
- MagicDNS internal `.oxide` resolution with OS resolver self-healing watchdog
- Seamless LAN mDNS discovery and mobile companion gateway bridging.

## Relationships

| Type | Target |
|------|--------|
| related | [LanDiscovery](/crates/oxide-network/src/lib/LanDiscovery.md) |
| related | [new](/crates/oxide-network/src/lib/new.md) |
| related | [broadcast_service](/crates/oxide-network/src/lib/broadcast_service.md) |
| related | [new](/crates/oxide-network/src/lib/new.md) |
| related | [broadcast_service](/crates/oxide-network/src/lib/broadcast_service.md) |
