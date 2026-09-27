---
okf_version: "0.2"
type: Function
title: pre_parse_packet
description: Zero-allocation stateless pre-decapsulation parser
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/pre_parse_packet
language: rust
---

# pre_parse_packet

Zero-allocation stateless pre-decapsulation parser

## Signature

```rust
pub fn pre_parse_packet(buffer: &[u8]) -> PreParseVerdict
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Zero-allocation stateless pre-decapsulation parser
[inline]

## Source
Lines 177–216 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
| called_by | [process_inbound_datagram](/crates/oxide-network/src/remote_access/process_inbound_datagram.md) |
