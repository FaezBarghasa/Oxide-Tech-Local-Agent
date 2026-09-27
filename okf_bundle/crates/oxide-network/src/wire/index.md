# wire

## Classs

- [AclActionWire](AclActionWire.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [AclDirectionWire](AclDirectionWire.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [AclRuleWire](AclRuleWire.md) — Wire format for ACL rule
- [AclUpdatePayload](AclUpdatePayload.md) — ACL update payload
- [BatchPacket](BatchPacket.md) — Batch packet for GRO/GSO aggregation
- [KeepalivePayload](KeepalivePayload.md) — Control keepalive payload
- [PacketHeader](PacketHeader.md) — Wire packet header (16 bytes, aligned for SIMD)
- [PathDiscoveryPayload](PathDiscoveryPayload.md) — Path discovery probe payload
- [PreParseVerdict](PreParseVerdict.md) — Pre-decapsulation packet classification verdict
- [RekeyNoticePayload](RekeyNoticePayload.md) — Rekey notice payload
- [ReplayWindow128](ReplayWindow128.md) — 128-bit sliding bitmask replay window (RFC 6479 inspired)
- [WirePacket](WirePacket.md) — Complete wire packet with header and payload

## Functions

- [check_and_update](check_and_update.md) — Validate sequence number and update the sliding window.
- [check_and_update](check_and_update_1.md) — Validate sequence number and update the sliding window.
- [default](default.md)
- [default](default_1.md)
- [from_bytes](from_bytes.md) — Parse from bytes
- [from_bytes](from_bytes_1.md) — Parse from bytes
- [is_control](is_control.md)
- [is_control](is_control_1.md)
- [last_sequence](last_sequence.md) — [inline]
- [last_sequence](last_sequence_1.md) — [inline]
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [new](new_6.md)
- [new](new_7.md)
- [packet_type](packet_type.md)
- [packet_type](packet_type_1.md)
- [packet_type](packet_type_2.md)
- [packet_type](packet_type_3.md)
- [pre_parse_packet](pre_parse_packet.md) — Zero-allocation stateless pre-decapsulation parser
- [test_pre_parse_junk_frame_filter](test_pre_parse_junk_frame_filter.md) — [test]
- [test_replay_window_anti_replay](test_replay_window_anti_replay.md) — [test]
- [test_wire_packet_roundtrip](test_wire_packet_roundtrip.md) — [test]
- [to_bytes](to_bytes.md) — Serialize to bytes
- [to_bytes](to_bytes_1.md) — Serialize to bytes
- [to_bytes](to_bytes_2.md)
- [to_bytes](to_bytes_3.md)
- [total_len](total_len.md)
- [total_len](total_len_1.md)
- [total_len](total_len_2.md)
- [total_len](total_len_3.md)
- [validate](validate.md)
- [validate](validate_1.md)
