---
okf_version: "0.2"
type: Class
title: TunConfig
description: Configuration parameters for virtual TUN device
resource: crates/oxide-network/src/tun.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:36Z"
concept_id: crates/oxide-network/src/tun/TunConfig
language: rust
---

# TunConfig

Configuration parameters for virtual TUN device

## Signature

```rust
pub struct TunConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration parameters for virtual TUN device
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `ip`
- `netmask`
- `mtu`
- `multi_queue`

## Source
Lines 12–18 in `crates/oxide-network/src/tun.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tun](/crates/oxide-network/src/tun.md) |
