---
okf_version: "0.2"
type: Function
title: load_f32
description: Load f32 tensor slice from disk with advisory reader lock and SIMD alignment verification.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/load_f32_1
language: rust
---

# load_f32

Load f32 tensor slice from disk with advisory reader lock and SIMD alignment verification.

## Signature

```rust
pub fn load_f32(path: P, offset: usize, count: usize) -> Result<Self, OxideError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Load f32 tensor slice from disk with advisory reader lock and SIMD alignment verification.

## Source
Lines 209–250 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
