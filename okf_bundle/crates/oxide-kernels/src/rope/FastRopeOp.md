---
okf_version: "0.2"
type: Class
title: FastRopeOp
description: Fast in-place Rotary Position Embedding (RoPE) operator.
resource: crates/oxide-kernels/src/rope.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:43:47Z"
concept_id: crates/oxide-kernels/src/rope/FastRopeOp
language: rust
---

# FastRopeOp

Fast in-place Rotary Position Embedding (RoPE) operator.

## Signature

```rust
pub struct FastRopeOp
```

## Visibility

- `pub`

## Docstring

Fast in-place Rotary Position Embedding (RoPE) operator.

Eliminates tensor cloning and intermediate matrix allocation by rotating
query and key pairs in-place within the same memory buffer.

## Methods

- `kernel_name`
- `head_dim`
- `theta_base`
- `cos_table`
- `sin_table`
- `max_seq_len`

## Source
Lines 7–14 in `crates/oxide-kernels/src/rope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rope](/crates/oxide-kernels/src/rope.md) |
