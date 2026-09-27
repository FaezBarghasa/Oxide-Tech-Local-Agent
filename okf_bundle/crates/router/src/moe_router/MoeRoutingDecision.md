---
okf_version: "0.2"
type: Class
title: MoeRoutingDecision
description: Routing decision output from the MoE Gating Network
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/MoeRoutingDecision
language: rust
---

# MoeRoutingDecision

Routing decision output from the MoE Gating Network

## Signature

```rust
pub struct MoeRoutingDecision
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

Routing decision output from the MoE Gating Network
[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `primary_expert`
- `secondary_expert`
- `routing_scores`
- `rationale`
- `complexity`
- `allocated_think_tokens`

## Source
Lines 144–151 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
