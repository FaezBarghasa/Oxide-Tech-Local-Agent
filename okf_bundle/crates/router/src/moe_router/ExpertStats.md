---
okf_version: "0.2"
type: Class
title: ExpertStats
description: Operational statistics tracked per expert for adaptive routing
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/ExpertStats
language: rust
---

# ExpertStats

Operational statistics tracked per expert for adaptive routing

## Signature

```rust
pub struct ExpertStats
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Operational statistics tracked per expert for adaptive routing
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `latency_ema_ms`
- `quality_ema`
- `consecutive_failures`
- `is_circuit_broken`

## Source
Lines 400–405 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
