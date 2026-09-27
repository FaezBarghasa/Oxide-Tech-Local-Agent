---
okf_version: "0.2"
type: Function
title: route
description: Calculate gating logits and select the best expert(s) for a given request
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/route_1
language: rust
---

# route

Calculate gating logits and select the best expert(s) for a given request

## Signature

```rust
pub fn route(&self, req: &InferenceRequest) -> MoeRoutingDecision
```

## Visibility

- `pub`

## Docstring

Calculate gating logits and select the best expert(s) for a given request

## Source
Lines 163–395 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
