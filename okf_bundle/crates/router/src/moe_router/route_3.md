---
okf_version: "0.2"
type: Function
title: route
description: "Route with adaptive adjustments based on health, latency EMA, and quality"
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/route_3
language: rust
---

# route

Route with adaptive adjustments based on health, latency EMA, and quality

## Signature

```rust
pub fn route(&self, req: &InferenceRequest) -> MoeRoutingDecision
```

## Visibility

- `pub`

## Docstring

Route with adaptive adjustments based on health, latency EMA, and quality

## Source
Lines 478–547 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
