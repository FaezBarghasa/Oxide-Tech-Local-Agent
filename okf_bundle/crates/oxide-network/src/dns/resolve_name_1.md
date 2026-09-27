---
okf_version: "0.2"
type: Function
title: resolve_name
description: Resolve hostname to Overlay IP
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns/resolve_name_1
language: rust
---

# resolve_name

Resolve hostname to Overlay IP

## Signature

```rust
pub fn resolve_name(&self, hostname: &str) -> Option<OverlayIp>
```

## Visibility

- `pub`

## Docstring

Resolve hostname to Overlay IP

## Source
Lines 74–77 in `crates/oxide-network/src/dns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dns](/crates/oxide-network/src/dns.md) |
