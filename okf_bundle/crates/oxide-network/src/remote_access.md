---
okf_version: "0.2"
type: Module
title: remote_access
description: "# Master Remote Access & Mesh Network Engine (`crates/oxide-network/src/remote_access.rs`)"
resource: crates/oxide-network/src/remote_access.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:54Z"
concept_id: crates/oxide-network/src/remote_access
language: rust
---

# remote_access

# Master Remote Access & Mesh Network Engine (`crates/oxide-network/src/remote_access.rs`)

## Docstring

# Master Remote Access & Mesh Network Engine (`crates/oxide-network/src/remote_access.rs`)

Orchestrates virtual TUN interfaces, dynamic TCP MSS clamping, lock-free RCU Radix routing,
Zero-Trust ACL rule enforcement, AEAD payload encryption, and MagicDNS self-healing resolution.

## Relationships

| Type | Target |
|------|--------|
| related | [PeerSession](/crates/oxide-network/src/remote_access/PeerSession.md) |
| related | [MeshStatusDto](/crates/oxide-network/src/remote_access/MeshStatusDto.md) |
| related | [RemoteAccessMeshEngine](/crates/oxide-network/src/remote_access/RemoteAccessMeshEngine.md) |
| related | [new](/crates/oxide-network/src/remote_access/new.md) |
| related | [node_id](/crates/oxide-network/src/remote_access/node_id.md) |
| related | [overlay_ip](/crates/oxide-network/src/remote_access/overlay_ip.md) |
| related | [tun](/crates/oxide-network/src/remote_access/tun.md) |
| related | [identity_key](/crates/oxide-network/src/remote_access/identity_key.md) |
| related | [dns_resolver](/crates/oxide-network/src/remote_access/dns_resolver.md) |
| related | [router](/crates/oxide-network/src/remote_access/router.md) |
| related | [connect_peer](/crates/oxide-network/src/remote_access/connect_peer.md) |
| related | [process_outbound_packet](/crates/oxide-network/src/remote_access/process_outbound_packet.md) |
| related | [process_inbound_datagram](/crates/oxide-network/src/remote_access/process_inbound_datagram.md) |
| related | [status](/crates/oxide-network/src/remote_access/status.md) |
| related | [new](/crates/oxide-network/src/remote_access/new.md) |
| related | [node_id](/crates/oxide-network/src/remote_access/node_id.md) |
| related | [overlay_ip](/crates/oxide-network/src/remote_access/overlay_ip.md) |
| related | [tun](/crates/oxide-network/src/remote_access/tun.md) |
| related | [identity_key](/crates/oxide-network/src/remote_access/identity_key.md) |
| related | [dns_resolver](/crates/oxide-network/src/remote_access/dns_resolver.md) |
| related | [router](/crates/oxide-network/src/remote_access/router.md) |
| related | [connect_peer](/crates/oxide-network/src/remote_access/connect_peer.md) |
| related | [process_outbound_packet](/crates/oxide-network/src/remote_access/process_outbound_packet.md) |
| related | [process_inbound_datagram](/crates/oxide-network/src/remote_access/process_inbound_datagram.md) |
| related | [status](/crates/oxide-network/src/remote_access/status.md) |
| related | [test_remote_mesh_engine_peer_connection_and_routing](/crates/oxide-network/src/remote_access/test_remote_mesh_engine_peer_connection_and_routing.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
