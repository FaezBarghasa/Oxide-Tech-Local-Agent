---
okf_version: "0.2"
type: Class
title: PacketHeader
description: "Wire packet header (16 bytes, aligned for SIMD)"
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/PacketHeader
language: rust
---

# PacketHeader

Wire packet header (16 bytes, aligned for SIMD)

## Signature

```rust
pub struct PacketHeader
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout)`

## Visibility

- `pub`

## Docstring

Wire packet header (16 bytes, aligned for SIMD)
[repr(C)]
[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout)]

## Methods

- `magic`
- `version`
- `packet_type`
- `flags`
- `packet_id`
- `payload_len`
- `reserved`

## Source
Lines 28–36 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
