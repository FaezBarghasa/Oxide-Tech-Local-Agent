---
okf_version: "0.2"
type: Class
title: FusedMoeRouterOp
description: Fused GPU/SIMD Mixture-of-Experts Router kernel for Gemma-4 and sparse MoE architectures.
resource: crates/oxide-kernels/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:50:32Z"
concept_id: crates/oxide-kernels/src/moe_router/FusedMoeRouterOp
language: rust
---

# FusedMoeRouterOp

Fused GPU/SIMD Mixture-of-Experts Router kernel for Gemma-4 and sparse MoE architectures.

## Signature

```rust
pub struct FusedMoeRouterOp
```

## Visibility

- `pub`

## Docstring

Fused GPU/SIMD Mixture-of-Experts Router kernel for Gemma-4 and sparse MoE architectures.

## Methods

- `kernel_name`
- `num_experts`
- `top_k`

## Source
Lines 17–21 in `crates/oxide-kernels/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/oxide-kernels/src/moe_router.md) |
