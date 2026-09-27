---
okf_version: "0.2"
type: Class
title: DplpmtudConfig
description: DPLPMTUD (Dynamic Packet-Layer Path MTU Discovery) Engine
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport/DplpmtudConfig
language: rust
---

# DplpmtudConfig

DPLPMTUD (Dynamic Packet-Layer Path MTU Discovery) Engine

## Signature

```rust
pub struct DplpmtudConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

DPLPMTUD (Dynamic Packet-Layer Path MTU Discovery) Engine
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `min_pmtu`
- `max_pmtu`
- `probe_interval_secs`

## Source
Lines 106–110 in `crates/oxide-network/src/transport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transport](/crates/oxide-network/src/transport.md) |
