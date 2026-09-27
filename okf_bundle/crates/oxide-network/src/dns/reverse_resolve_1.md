---
okf_version: "0.2"
type: Function
title: reverse_resolve
description: Reverse-resolve Overlay IP to hostname
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/reverse_resolve_1
language: rust
---

# reverse_resolve

Reverse-resolve Overlay IP to hostname

## Signature

```rust
pub fn reverse_resolve(&self, ip: OverlayIp) -> Option<String>
```

## Visibility

- `pub`

## Docstring

Reverse-resolve Overlay IP to hostname

## Source
Lines 80–82 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
