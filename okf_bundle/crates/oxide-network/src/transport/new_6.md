---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/new_6
language: rust
---

# new

## Signature

```rust
impl QuicMeshTransport { pub fn new(
        port_config: PortHopperConfig,
        dplpmtud_config: DplpmtudConfig,
        cb_config: CircuitBreakerConfig,
    ) -> (Self, broadcast::Receiver<TransportEvent>) }
```

## Visibility

- `pub`

## Source
Lines 229–244 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
