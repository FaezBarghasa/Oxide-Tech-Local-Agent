---
okf_version: "0.2"
type: Class
title: RouteTarget
description: Target destination for routed overlay packets
resource: crates/oxide-network/src/routing.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:25Z"
concept_id: crates/oxide-network/src/routing/RouteTarget
language: rust
---

# RouteTarget

Target destination for routed overlay packets

## Signature

```rust
pub enum RouteTarget
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Target destination for routed overlay packets
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Methods

- `node_id`
- `endpoint`
- `router_id`
- `gateway_ip`
- `relay_id`

## Source
Lines 14–29 in `crates/oxide-network/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-network/src/routing.md) |
