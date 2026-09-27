---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/new_3
language: rust
---

# new

## Signature

```rust
pub fn new(packet_type: PacketType, packet_id: u32, payload: Vec<u8>) -> Result<Self, OxideError>
```

## Visibility

- `pub`

## Source
Lines 85–93 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
