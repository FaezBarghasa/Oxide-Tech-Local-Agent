---
okf_version: "0.2"
type: Function
title: register
description: Register a host mapping in MagicDNS
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/register_1
language: rust
---

# register

Register a host mapping in MagicDNS

## Signature

```rust
pub fn register(&self, hostname: &str, ip: OverlayIp, node_id: NodeId)
```

## Visibility

- `pub`

## Docstring

Register a host mapping in MagicDNS

## Source
Lines 53–63 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
