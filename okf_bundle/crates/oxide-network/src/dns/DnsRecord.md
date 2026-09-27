---
okf_version: "0.2"
type: Class
title: DnsRecord
description: Record representing a MagicDNS name mapping
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/DnsRecord
language: rust
---

# DnsRecord

Record representing a MagicDNS name mapping

## Signature

```rust
pub struct DnsRecord
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Record representing a MagicDNS name mapping
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `hostname`
- `ip`
- `node_id`
- `ttl`

## Source
Lines 14–19 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
