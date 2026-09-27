---
okf_version: "0.2"
type: Function
title: compress_block_avx512
description: Vectorized AVX-512 block compression (32 floats = two 512-bit ZMM registers or 16-element chunks)
resource: crates/oxide-kernels/src/avx512_compress.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:12:04Z"
concept_id: crates/oxide-kernels/src/avx512_compress/compress_block_avx512_1
language: rust
---

# compress_block_avx512

Vectorized AVX-512 block compression (32 floats = two 512-bit ZMM registers or 16-element chunks)

## Signature

```rust
fn compress_block_avx512(&self, chunk: &[f32]) -> CompressedBlockInt8
```

## Decorators

- `cfg(target_arch = "x86_64")`
- `target_feature(enable = "avx512f,avx512bw")`

## Docstring

Vectorized AVX-512 block compression (32 floats = two 512-bit ZMM registers or 16-element chunks)
[cfg(target_arch = "x86_64")]
[target_feature(enable = "avx512f,avx512bw")]

## Source
Lines 126–162 in `crates/oxide-kernels/src/avx512_compress.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [avx512_compress](/crates/oxide-kernels/src/avx512_compress.md) |
