---
okf_version: "0.2"
type: Class
title: MoeRoutingPlan
description: Output of the Fused MoE Router kernel containing routing indices and normalized expert weights.
resource: crates/oxide-kernels/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:50:32Z"
concept_id: crates/oxide-kernels/src/moe_router/MoeRoutingPlan
language: rust
---

# MoeRoutingPlan

Output of the Fused MoE Router kernel containing routing indices and normalized expert weights.

## Signature

```rust
pub struct MoeRoutingPlan
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Output of the Fused MoE Router kernel containing routing indices and normalized expert weights.
[derive(Debug, Clone, PartialEq)]

## Methods

- `num_tokens`
- `selected_experts`
- `routing_weights`
- `expert_dispatches`

## Source
Lines 5–14 in `crates/oxide-kernels/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/oxide-kernels/src/moe_router.md) |
