---
okf_version: "0.2"
type: Function
title: route_tokens
description: "Perform fused Top-K selection, Softmax normalization, and token-to-expert dispatch routing."
resource: crates/oxide-kernels/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:50:32Z"
concept_id: crates/oxide-kernels/src/moe_router/route_tokens_1
language: rust
---

# route_tokens

Perform fused Top-K selection, Softmax normalization, and token-to-expert dispatch routing.

## Signature

```rust
pub fn route_tokens(
        &self,
        router_logits: &[f32],
        num_tokens: usize,
    ) -> Result<MoeRoutingPlan, OxideError>
```

## Visibility

- `pub`

## Docstring

Perform fused Top-K selection, Softmax normalization, and token-to-expert dispatch routing.

`router_logits` has dimensions `num_tokens * num_experts`.

## Source
Lines 45–115 in `crates/oxide-kernels/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/oxide-kernels/src/moe_router.md) |
