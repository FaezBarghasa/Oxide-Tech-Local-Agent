---
okf_version: "0.2"
type: Class
title: CompressedBlockInt8
description: Compressed INT8 Block with block-wise FP32 scaling factor
resource: crates/oxide-kernels/src/avx512_compress.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:12:04Z"
concept_id: crates/oxide-kernels/src/avx512_compress/CompressedBlockInt8
language: rust
---

# CompressedBlockInt8

Compressed INT8 Block with block-wise FP32 scaling factor

## Signature

```rust
pub struct CompressedBlockInt8
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Compressed INT8 Block with block-wise FP32 scaling factor
[derive(Debug, Clone, PartialEq)]

## Methods

- `scale`
- `data`

## Source
Lines 22–25 in `crates/oxide-kernels/src/avx512_compress.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [avx512_compress](/crates/oxide-kernels/src/avx512_compress.md) |
