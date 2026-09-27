---
okf_version: "0.2"
type: Function
title: is_avx512_supported
description: Check whether AVX-512F (Foundation) and AVX-512BW (Byte/Word) are supported by host CPU.
resource: crates/oxide-kernels/src/avx512_compress.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:12:04Z"
concept_id: crates/oxide-kernels/src/avx512_compress/is_avx512_supported
language: rust
---

# is_avx512_supported

Check whether AVX-512F (Foundation) and AVX-512BW (Byte/Word) are supported by host CPU.

## Signature

```rust
impl Avx512Compressor { pub fn is_avx512_supported() -> bool }
```

## Visibility

- `pub`

## Docstring

Check whether AVX-512F (Foundation) and AVX-512BW (Byte/Word) are supported by host CPU.

## Source
Lines 33–42 in `crates/oxide-kernels/src/avx512_compress.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [avx512_compress](/crates/oxide-kernels/src/avx512_compress.md) |
