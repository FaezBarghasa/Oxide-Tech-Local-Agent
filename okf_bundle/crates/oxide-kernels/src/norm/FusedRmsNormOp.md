---
okf_version: "0.2"
type: Class
title: FusedRmsNormOp
description: Fused Root-Mean-Square Normalization (RMSNorm) operator with optional fused residual addition.
resource: crates/oxide-kernels/src/norm.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:35:12Z"
concept_id: crates/oxide-kernels/src/norm/FusedRmsNormOp
language: rust
---

# FusedRmsNormOp

Fused Root-Mean-Square Normalization (RMSNorm) operator with optional fused residual addition.

## Signature

```rust
pub struct FusedRmsNormOp
```

## Visibility

- `pub`

## Docstring

Fused Root-Mean-Square Normalization (RMSNorm) operator with optional fused residual addition.

Combines residual accumulation, variance reduction, and weight scaling into
a single memory pass, eliminating redundant DRAM reads and writes.

## Methods

- `kernel_name`
- `eps`

## Source
Lines 7–10 in `crates/oxide-kernels/src/norm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [norm](/crates/oxide-kernels/src/norm.md) |
