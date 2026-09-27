---
okf_version: "0.2"
type: Function
title: fast_hadamard_transform_inplace
description: Fast in-place blockwise Walsh-Hadamard Transform (FWHT) for power-of-two blocks.
resource: crates/oxide-kernels/src/ternary.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-kernels/src/ternary/fast_hadamard_transform_inplace
language: rust
---

# fast_hadamard_transform_inplace

Fast in-place blockwise Walsh-Hadamard Transform (FWHT) for power-of-two blocks.

## Signature

```rust
impl TernaryHadamardOp { pub fn fast_hadamard_transform_inplace(&self, block: &mut [f32]) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Fast in-place blockwise Walsh-Hadamard Transform (FWHT) for power-of-two blocks.

The orthogonal Hadamard rotation spreads activation outliers before ternary quantization,
eliminating the reasoning collapse in 1-bit models like Ternary-Bonsai.

## Source
Lines 89–118 in `crates/oxide-kernels/src/ternary.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ternary](/crates/oxide-kernels/src/ternary.md) |
