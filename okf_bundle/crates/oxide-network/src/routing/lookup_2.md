---
okf_version: "0.2"
type: Function
title: lookup
description: "[inline]"
resource: crates/oxide-network/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:25Z"
concept_id: crates/oxide-network/src/routing/lookup_2
language: rust
---

# lookup

[inline]

## Signature

```rust
impl RcuRouter { pub fn lookup(&self, ip: OverlayIp) -> Option<RouteTarget> }
```

## Visibility

- `pub`

## Docstring

[inline]

## Source
Lines 173–176 in `crates/oxide-network/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-network/src/routing.md) |
