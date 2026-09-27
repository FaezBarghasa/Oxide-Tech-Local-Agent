---
okf_version: "0.2"
type: Function
title: forward_fused_residual
description: "Fused forward: x = x + residual, then apply RMSNorm and store in output."
resource: crates/oxide-kernels/src/norm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:35:12Z"
concept_id: crates/oxide-kernels/src/norm/forward_fused_residual_1
language: rust
---

# forward_fused_residual

Fused forward: x = x + residual, then apply RMSNorm and store in output.

## Signature

```rust
pub fn forward_fused_residual(
        &self,
        x: &[f32],
        residual: &mut [f32],
        weight: &[f32],
        output: &mut [f32],
    ) -> Result<(), OxideError>
```

## Visibility

- `pub`

## Docstring

Fused forward: x = x + residual, then apply RMSNorm and store in output.

## Source
Lines 55–86 in `crates/oxide-kernels/src/norm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [norm](/crates/oxide-kernels/src/norm.md) |
