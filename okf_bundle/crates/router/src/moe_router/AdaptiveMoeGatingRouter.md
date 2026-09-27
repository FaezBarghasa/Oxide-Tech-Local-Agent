---
okf_version: "0.2"
type: Class
title: AdaptiveMoeGatingRouter
description: "Adaptive MoE Gating Router with dynamic feedback (latency EMA, quality scoring, circuit breaking)."
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/AdaptiveMoeGatingRouter
language: rust
---

# AdaptiveMoeGatingRouter

Adaptive MoE Gating Router with dynamic feedback (latency EMA, quality scoring, circuit breaking).

## Signature

```rust
pub struct AdaptiveMoeGatingRouter
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Adaptive MoE Gating Router with dynamic feedback (latency EMA, quality scoring, circuit breaking).
[derive(Debug, Clone, Default)]

## Methods

- `inner_router`
- `stats`
- `circuit_breaker_threshold`

## Source
Lines 420–424 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
