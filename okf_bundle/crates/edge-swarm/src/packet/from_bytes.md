---
okf_version: "0.2"
type: Function
title: from_bytes
resource: crates/edge-swarm/src/packet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:edge-swarm"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:51:10Z"
concept_id: crates/edge-swarm/src/packet/from_bytes
language: rust
---

# from_bytes

## Signature

```rust
impl HardwareTelemetryPacket { pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> }
```

## Visibility

- `pub`

## Source
Lines 20–22 in `crates/edge-swarm/src/packet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [packet](/crates/edge-swarm/src/packet.md) |
