---
okf_version: "0.2"
type: Function
title: dequantize_and_rotate
description: "Combined pipeline: unpacks trits, multiplies by group scales, and applies Hadamard rotation."
resource: crates/oxide-kernels/src/ternary.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-kernels/src/ternary/dequantize_and_rotate
language: rust
---

# dequantize_and_rotate

Combined pipeline: unpacks trits, multiplies by group scales, and applies Hadamard rotation.

## Signature

```rust
impl TernaryHadamardOp { pub fn dequantize_and_rotate(
        &self,
        packed_trits: &[u8],
        scales: &[f32],
        output: &mut [f32],
    ) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Combined pipeline: unpacks trits, multiplies by group scales, and applies Hadamard rotation.

## Source
Lines 121–136 in `crates/oxide-kernels/src/ternary.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ternary](/crates/oxide-kernels/src/ternary.md) |
