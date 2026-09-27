---
okf_version: "0.2"
type: Function
title: list_records
description: List all registered records
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/list_records
language: rust
---

# list_records

List all registered records

## Signature

```rust
impl MagicDnsResolver { pub fn list_records(&self) -> Vec<DnsRecord> }
```

## Visibility

- `pub`

## Docstring

List all registered records

## Source
Lines 85–90 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
