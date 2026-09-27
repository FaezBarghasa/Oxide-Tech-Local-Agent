---
okf_version: "0.2"
type: Function
title: apply_inplace
description: "Apply RoPE rotation in-place to a token embedding slice at position `pos`."
resource: crates/oxide-kernels/src/rope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:43:47Z"
concept_id: crates/oxide-kernels/src/rope/apply_inplace
language: rust
---

# apply_inplace

Apply RoPE rotation in-place to a token embedding slice at position `pos`.

## Signature

```rust
impl FastRopeOp { pub fn apply_inplace(&self, slice: &mut [f32], pos: usize) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Apply RoPE rotation in-place to a token embedding slice at position `pos`.

Slice length must equal `head_dim`.

## Source
Lines 52–86 in `crates/oxide-kernels/src/rope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rope](/crates/oxide-kernels/src/rope.md) |
