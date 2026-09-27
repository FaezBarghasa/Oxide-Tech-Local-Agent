---
okf_version: "0.2"
type: Function
title: forward
description: "Apply in-place RMSNorm:"
resource: crates/oxide-kernels/src/norm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:35:12Z"
concept_id: crates/oxide-kernels/src/norm/forward
language: rust
---

# forward

Apply in-place RMSNorm:

## Signature

```rust
impl FusedRmsNormOp { pub fn forward(&self, x: &mut [f32], weight: &[f32]) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Apply in-place RMSNorm:
x = (x / sqrt(mean(x^2) + eps)) * weight

## Source
Lines 31–52 in `crates/oxide-kernels/src/norm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [norm](/crates/oxide-kernels/src/norm.md) |
