---
okf_version: "0.2"
type: Function
title: supports_fp8_mma
description: Whether native FP8 / FP4 tensor cores are available
resource: crates/oxide-kernels/src/autotune.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:32Z"
concept_id: crates/oxide-kernels/src/autotune/supports_fp8_mma_1
language: rust
---

# supports_fp8_mma

Whether native FP8 / FP4 tensor cores are available

## Signature

```rust
pub fn supports_fp8_mma(&self) -> bool
```

## Visibility

- `pub`

## Docstring

Whether native FP8 / FP4 tensor cores are available

## Source
Lines 52–60 in `crates/oxide-kernels/src/autotune.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autotune](/crates/oxide-kernels/src/autotune.md) |
