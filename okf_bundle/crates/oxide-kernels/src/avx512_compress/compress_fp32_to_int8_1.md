---
okf_version: "0.2"
type: Function
title: compress_fp32_to_int8
description: Vectorized compression of FP32 slice into INT8 blocks with dynamic scaling.
resource: crates/oxide-kernels/src/avx512_compress.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:12:04Z"
concept_id: crates/oxide-kernels/src/avx512_compress/compress_fp32_to_int8_1
language: rust
---

# compress_fp32_to_int8

Vectorized compression of FP32 slice into INT8 blocks with dynamic scaling.

## Signature

```rust
pub fn compress_fp32_to_int8(
        &self,
        src: &[f32],
    ) -> Result<Vec<CompressedBlockInt8>, OxideError>
```

## Visibility

- `pub`

## Docstring

Vectorized compression of FP32 slice into INT8 blocks with dynamic scaling.

## Source
Lines 45–70 in `crates/oxide-kernels/src/avx512_compress.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [avx512_compress](/crates/oxide-kernels/src/avx512_compress.md) |
