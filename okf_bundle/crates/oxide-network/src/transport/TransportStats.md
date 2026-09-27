---
okf_version: "0.2"
type: Class
title: TransportStats
description: Dynamic Statistics for Transport Layer
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/TransportStats
language: rust
---

# TransportStats

Dynamic Statistics for Transport Layer

## Signature

```rust
pub struct TransportStats
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Dynamic Statistics for Transport Layer
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `active_connections`
- `total_connections`
- `datagrams_sent`
- `datagrams_received`
- `bytes_sent`
- `bytes_received`
- `connection_errors`
- `pmtu_probes_sent`

## Source
Lines 37–46 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
