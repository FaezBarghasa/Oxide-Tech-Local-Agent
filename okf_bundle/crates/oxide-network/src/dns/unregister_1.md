---
okf_version: "0.2"
type: Function
title: unregister
description: Deregister a host mapping
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/unregister_1
language: rust
---

# unregister

Deregister a host mapping

## Signature

```rust
pub fn unregister(&self, hostname: &str)
```

## Visibility

- `pub`

## Docstring

Deregister a host mapping

## Source
Lines 66–71 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
