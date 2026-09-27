---
okf_version: "0.2"
type: Class
title: PacketType
description: Packet discriminator for wire protocol multiplexing
resource: crates/oxide-network/src/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:05:47Z"
concept_id: crates/oxide-network/src/types/PacketType
language: rust
---

# PacketType

Packet discriminator for wire protocol multiplexing

## Signature

```rust
pub enum PacketType
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `repr(u8)`

## Visibility

- `pub`

## Docstring

Packet discriminator for wire protocol multiplexing
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[repr(u8)]

## Source
Lines 337–349 in `crates/oxide-network/src/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-network/src/types.md) |
