---
okf_version: "0.2"
type: Module
title: types
description: "# Core Types for Zero-Trust Remote Mesh Network (`crates/oxide-network/src/types.rs`)"
resource: crates/oxide-network/src/types.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:05:47Z"
concept_id: crates/oxide-network/src/types
language: rust
---

# types

# Core Types for Zero-Trust Remote Mesh Network (`crates/oxide-network/src/types.rs`)

## Docstring

# Core Types for Zero-Trust Remote Mesh Network (`crates/oxide-network/src/types.rs`)

Provides fundamental types for nodes, mesh domains, overlay addresses, and wire discriminators.

## Relationships

| Type | Target |
|------|--------|
| related | [NodeId](/crates/oxide-network/src/types/NodeId.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [from_uuid](/crates/oxide-network/src/types/from_uuid.md) |
| related | [as_uuid](/crates/oxide-network/src/types/as_uuid.md) |
| related | [as_bytes](/crates/oxide-network/src/types/as_bytes.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [from_uuid](/crates/oxide-network/src/types/from_uuid.md) |
| related | [as_uuid](/crates/oxide-network/src/types/as_uuid.md) |
| related | [as_bytes](/crates/oxide-network/src/types/as_bytes.md) |
| related | [default](/crates/oxide-network/src/types/default.md) |
| related | [default](/crates/oxide-network/src/types/default.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [MeshName](/crates/oxide-network/src/types/MeshName.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [as_str](/crates/oxide-network/src/types/as_str.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [as_str](/crates/oxide-network/src/types/as_str.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [OverlayIp](/crates/oxide-network/src/types/OverlayIp.md) |
| related | [is_v4](/crates/oxide-network/src/types/is_v4.md) |
| related | [is_v6](/crates/oxide-network/src/types/is_v6.md) |
| related | [as_ip_addr](/crates/oxide-network/src/types/as_ip_addr.md) |
| related | [default_v4](/crates/oxide-network/src/types/default_v4.md) |
| related | [default_v6](/crates/oxide-network/src/types/default_v6.md) |
| related | [is_v4](/crates/oxide-network/src/types/is_v4.md) |
| related | [is_v6](/crates/oxide-network/src/types/is_v6.md) |
| related | [as_ip_addr](/crates/oxide-network/src/types/as_ip_addr.md) |
| related | [default_v4](/crates/oxide-network/src/types/default_v4.md) |
| related | [default_v6](/crates/oxide-network/src/types/default_v6.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [from](/crates/oxide-network/src/types/from.md) |
| related | [OverlayPrefix](/crates/oxide-network/src/types/OverlayPrefix.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [contains](/crates/oxide-network/src/types/contains.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [contains](/crates/oxide-network/src/types/contains.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [ProtocolVersion](/crates/oxide-network/src/types/ProtocolVersion.md) |
| related | [TransportProtocol](/crates/oxide-network/src/types/TransportProtocol.md) |
| related | [NodeCapabilities](/crates/oxide-network/src/types/NodeCapabilities.md) |
| related | [default](/crates/oxide-network/src/types/default.md) |
| related | [default](/crates/oxide-network/src/types/default.md) |
| related | [Endpoint](/crates/oxide-network/src/types/Endpoint.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [socket_addr](/crates/oxide-network/src/types/socket_addr.md) |
| related | [new](/crates/oxide-network/src/types/new.md) |
| related | [socket_addr](/crates/oxide-network/src/types/socket_addr.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [fmt](/crates/oxide-network/src/types/fmt.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [from_str](/crates/oxide-network/src/types/from_str.md) |
| related | [PacketType](/crates/oxide-network/src/types/PacketType.md) |
| related | [try_from](/crates/oxide-network/src/types/try_from.md) |
| related | [try_from](/crates/oxide-network/src/types/try_from.md) |
| related | [test_node_id_uniqueness_and_serialization](/crates/oxide-network/src/types/test_node_id_uniqueness_and_serialization.md) |
| related | [test_overlay_prefix_contains](/crates/oxide-network/src/types/test_overlay_prefix_contains.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
