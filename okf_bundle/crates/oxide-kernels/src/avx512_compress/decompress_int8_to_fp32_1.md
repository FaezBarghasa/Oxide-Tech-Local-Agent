---
okf_version: "0.2"
type: Function
title: decompress_int8_to_fp32
description: Vectorized decompression of INT8 blocks back into FP32 buffer.
resource: crates/oxide-kernels/src/avx512_compress.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:12:04Z"
concept_id: crates/oxide-kernels/src/avx512_compress/decompress_int8_to_fp32_1
language: rust
---

# decompress_int8_to_fp32

Vectorized decompression of INT8 blocks back into FP32 buffer.

## Signature

```rust
pub fn decompress_int8_to_fp32(
        &self,
        blocks: &[CompressedBlockInt8],
        dst: &mut [f32],
    ) -> Result<(), OxideError>
```

## Visibility

- `pub`

## Docstring

Vectorized decompression of INT8 blocks back into FP32 buffer.

## Source
Lines 73–107 in `crates/oxide-kernels/src/avx512_compress.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [avx512_compress](/crates/oxide-kernels/src/avx512_compress.md) |
