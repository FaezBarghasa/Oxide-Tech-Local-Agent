---
okf_version: "0.2"
type: Function
title: packet_type
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/packet_type
language: rust
---

# packet_type

## Signature

```rust
impl PacketHeader { pub fn packet_type(&self) -> Result<PacketType, OxideError> }
```

## Visibility

- `pub`

## Source
Lines 71–74 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
