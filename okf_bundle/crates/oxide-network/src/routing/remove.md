---
okf_version: "0.2"
type: Function
title: remove
resource: crates/oxide-network/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:25Z"
concept_id: crates/oxide-network/src/routing/remove
language: rust
---

# remove

## Signature

```rust
impl RadixRoutingTable { pub fn remove(&mut self, prefix: &OverlayPrefix) -> bool }
```

## Visibility

- `pub`

## Source
Lines 66–70 in `crates/oxide-network/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-network/src/routing.md) |
