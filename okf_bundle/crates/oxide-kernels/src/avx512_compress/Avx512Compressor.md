---
okf_version: "0.2"
type: Class
title: Avx512Compressor
description: Vectorized AVX-512 Tensor / Weight Compression Engine.
resource: crates/oxide-kernels/src/avx512_compress.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:12:04Z"
concept_id: crates/oxide-kernels/src/avx512_compress/Avx512Compressor
language: rust
---

# Avx512Compressor

Vectorized AVX-512 Tensor / Weight Compression Engine.

## Signature

```rust
pub struct Avx512Compressor
```

## Visibility

- `pub`

## Docstring

Vectorized AVX-512 Tensor / Weight Compression Engine.

Features:
1. FP32 -> INT8 symmetric block quantization (32 floats per block, AVX-512F / AVX-512BW).
2. INT8 -> FP32 vectorized decompression with fused scale multiplication.
3. FP32 -> FP16 / BF16 truncation & packing (AVX-512_FP16 / AVX-512_BF16 semantics).
4. Dynamic CPU target feature detection with automatic scalar fallback.

## Methods

- `block_size`

## Source
Lines 10–12 in `crates/oxide-kernels/src/avx512_compress.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [avx512_compress](/crates/oxide-kernels/src/avx512_compress.md) |
