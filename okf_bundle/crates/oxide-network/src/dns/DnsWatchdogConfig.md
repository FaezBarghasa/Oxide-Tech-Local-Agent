---
okf_version: "0.2"
type: Class
title: DnsWatchdogConfig
description: Watchdog checking DNS resolution health
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/DnsWatchdogConfig
language: rust
---

# DnsWatchdogConfig

Watchdog checking DNS resolution health

## Signature

```rust
pub struct DnsWatchdogConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Watchdog checking DNS resolution health
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `check_interval_secs`
- `self_heal`

## Source
Lines 95–98 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
