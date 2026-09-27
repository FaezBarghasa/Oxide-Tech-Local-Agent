---
okf_version: "0.2"
type: Function
title: remove_route
resource: crates/oxide-network/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:25Z"
concept_id: crates/oxide-network/src/routing/remove_route
language: rust
---

# remove_route

## Signature

```rust
impl RcuRouter { pub fn remove_route(&self, prefix: &OverlayPrefix) -> bool }
```

## Visibility

- `pub`

## Source
Lines 185–193 in `crates/oxide-network/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-network/src/routing.md) |
