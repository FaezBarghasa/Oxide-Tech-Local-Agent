---
okf_version: "0.2"
type: Function
title: decompress_block_avx512
description: Vectorized AVX-512 block decompression
resource: crates/oxide-kernels/src/avx512_compress.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:12:04Z"
concept_id: crates/oxide-kernels/src/avx512_compress/decompress_block_avx512
language: rust
---

# decompress_block_avx512

Vectorized AVX-512 block decompression

## Signature

```rust
impl Avx512Compressor { fn decompress_block_avx512(&self, block: &CompressedBlockInt8, dst: &mut [f32]) }
```

## Docstring

Vectorized AVX-512 block decompression
[cfg(target_arch = "x86_64")]
[target_feature(enable = "avx512f,avx512bw")]

## Source
Lines 167–190 in `crates/oxide-kernels/src/avx512_compress.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [avx512_compress](/crates/oxide-kernels/src/avx512_compress.md) |
