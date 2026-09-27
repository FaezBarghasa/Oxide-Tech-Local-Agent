---
okf_version: "0.2"
type: Function
title: new
description: Construct a new RoPE operator with precomputed trigonometric tables.
resource: crates/oxide-kernels/src/rope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:43:47Z"
concept_id: crates/oxide-kernels/src/rope/new_1
language: rust
---

# new

Construct a new RoPE operator with precomputed trigonometric tables.

## Signature

```rust
pub fn new(head_dim: usize, max_seq_len: usize, theta_base: f32) -> Result<Self, OxideError>
```

## Visibility

- `pub`

## Docstring

Construct a new RoPE operator with precomputed trigonometric tables.

## Source
Lines 18–47 in `crates/oxide-kernels/src/rope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rope](/crates/oxide-kernels/src/rope.md) |
