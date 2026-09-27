---
okf_version: "0.2"
type: Module
title: wire
description: "# Wire Protocol Framing for Data & Control Plane Packets (`crates/oxide-network/src/wire.rs`)"
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire
language: rust
---

# wire

# Wire Protocol Framing for Data & Control Plane Packets (`crates/oxide-network/src/wire.rs`)

## Docstring

# Wire Protocol Framing for Data & Control Plane Packets (`crates/oxide-network/src/wire.rs`)

Provides binary envelope encoding, 128-bit anti-replay protection,
GRO/GSO batching, and zero-allocation anti-DPI packet pre-parsing.

## Relationships

| Type | Target |
|------|--------|
| related | [PacketHeader](/crates/oxide-network/src/wire/PacketHeader.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [validate](/crates/oxide-network/src/wire/validate.md) |
| related | [packet_type](/crates/oxide-network/src/wire/packet_type.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [validate](/crates/oxide-network/src/wire/validate.md) |
| related | [packet_type](/crates/oxide-network/src/wire/packet_type.md) |
| related | [WirePacket](/crates/oxide-network/src/wire/WirePacket.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [total_len](/crates/oxide-network/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-network/src/wire/to_bytes.md) |
| related | [from_bytes](/crates/oxide-network/src/wire/from_bytes.md) |
| related | [packet_type](/crates/oxide-network/src/wire/packet_type.md) |
| related | [is_control](/crates/oxide-network/src/wire/is_control.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [total_len](/crates/oxide-network/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-network/src/wire/to_bytes.md) |
| related | [from_bytes](/crates/oxide-network/src/wire/from_bytes.md) |
| related | [packet_type](/crates/oxide-network/src/wire/packet_type.md) |
| related | [is_control](/crates/oxide-network/src/wire/is_control.md) |
| related | [BatchPacket](/crates/oxide-network/src/wire/BatchPacket.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [total_len](/crates/oxide-network/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-network/src/wire/to_bytes.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [total_len](/crates/oxide-network/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-network/src/wire/to_bytes.md) |
| related | [PreParseVerdict](/crates/oxide-network/src/wire/PreParseVerdict.md) |
| related | [pre_parse_packet](/crates/oxide-network/src/wire/pre_parse_packet.md) |
| related | [ReplayWindow128](/crates/oxide-network/src/wire/ReplayWindow128.md) |
| related | [default](/crates/oxide-network/src/wire/default.md) |
| related | [default](/crates/oxide-network/src/wire/default.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [check_and_update](/crates/oxide-network/src/wire/check_and_update.md) |
| related | [last_sequence](/crates/oxide-network/src/wire/last_sequence.md) |
| related | [new](/crates/oxide-network/src/wire/new.md) |
| related | [check_and_update](/crates/oxide-network/src/wire/check_and_update.md) |
| related | [last_sequence](/crates/oxide-network/src/wire/last_sequence.md) |
| related | [KeepalivePayload](/crates/oxide-network/src/wire/KeepalivePayload.md) |
| related | [PathDiscoveryPayload](/crates/oxide-network/src/wire/PathDiscoveryPayload.md) |
| related | [RekeyNoticePayload](/crates/oxide-network/src/wire/RekeyNoticePayload.md) |
| related | [AclUpdatePayload](/crates/oxide-network/src/wire/AclUpdatePayload.md) |
| related | [AclRuleWire](/crates/oxide-network/src/wire/AclRuleWire.md) |
| related | [AclActionWire](/crates/oxide-network/src/wire/AclActionWire.md) |
| related | [AclDirectionWire](/crates/oxide-network/src/wire/AclDirectionWire.md) |
| related | [test_wire_packet_roundtrip](/crates/oxide-network/src/wire/test_wire_packet_roundtrip.md) |
| related | [test_replay_window_anti_replay](/crates/oxide-network/src/wire/test_replay_window_anti_replay.md) |
| related | [test_pre_parse_junk_frame_filter](/crates/oxide-network/src/wire/test_pre_parse_junk_frame_filter.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [zerocopy](/_dependencies/cargo/zerocopy.md) |
