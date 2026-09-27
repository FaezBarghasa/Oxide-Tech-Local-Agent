---
okf_version: "0.2"
type: Function
title: to_bytes
description: Zero-heap binary serialization via postcard for direct UART / RTT streaming.
resource: crates/edge-swarm/src/packet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:edge-swarm"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:51:10Z"
concept_id: crates/edge-swarm/src/packet/to_bytes
language: rust
---

# to_bytes

Zero-heap binary serialization via postcard for direct UART / RTT streaming.

## Signature

```rust
impl HardwareTelemetryPacket { pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> }
```

## Visibility

- `pub`

## Docstring

Zero-heap binary serialization via postcard for direct UART / RTT streaming.

## Source
Lines 16–18 in `crates/edge-swarm/src/packet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [packet](/crates/edge-swarm/src/packet.md) |
