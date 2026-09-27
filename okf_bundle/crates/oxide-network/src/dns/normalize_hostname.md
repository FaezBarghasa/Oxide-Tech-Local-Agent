---
okf_version: "0.2"
type: Function
title: normalize_hostname
description: "Normalizes hostname to lowercase `.oxide` suffix"
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/normalize_hostname
language: rust
---

# normalize_hostname

Normalizes hostname to lowercase `.oxide` suffix

## Signature

```rust
impl MagicDnsResolver { pub fn normalize_hostname(name: &str) -> String }
```

## Visibility

- `pub`

## Docstring

Normalizes hostname to lowercase `.oxide` suffix

## Source
Lines 43–50 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
