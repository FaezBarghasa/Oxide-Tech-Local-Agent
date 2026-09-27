---
okf_version: "0.2"
type: Function
title: broadcast_service
resource: crates/oxide-network/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:07:35Z"
concept_id: crates/oxide-network/src/lib/broadcast_service
language: rust
---

# broadcast_service

## Signature

```rust
impl LanDiscovery { pub fn broadcast_service(&self, port: u16) -> Result<(), mdns_sd::Error> }
```

## Visibility

- `pub`

## Source
Lines 61–78 in `crates/oxide-network/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-network/src/lib.md) |
