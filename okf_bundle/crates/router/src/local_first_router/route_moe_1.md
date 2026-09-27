---
okf_version: "0.2"
type: Function
title: route_moe
description: "Select optimal expert among Gemma-4-26B-A4B, qwen3.8-27b, Ornith-1.5-35B, and TurboFCFusion"
resource: crates/router/src/local_first_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/local_first_router/route_moe_1
language: rust
---

# route_moe

Select optimal expert among Gemma-4-26B-A4B, qwen3.8-27b, Ornith-1.5-35B, and TurboFCFusion

## Signature

```rust
pub fn route_moe(&self, req: &InferenceRequest) -> MoeRoutingDecision
```

## Visibility

- `pub`

## Docstring

Select optimal expert among Gemma-4-26B-A4B, qwen3.8-27b, Ornith-1.5-35B, and TurboFCFusion

## Source
Lines 148–150 in `crates/router/src/local_first_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_first_router](/crates/router/src/local_first_router.md) |
