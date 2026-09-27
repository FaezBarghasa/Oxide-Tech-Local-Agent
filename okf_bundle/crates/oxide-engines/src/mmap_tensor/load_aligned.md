---
okf_version: "0.2"
type: Function
title: load_aligned
description: "Load generic typed tensor slice enforcing >= 64-byte SIMD alignment and shared advisory read lock."
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/load_aligned
language: rust
---

# load_aligned

Load generic typed tensor slice enforcing >= 64-byte SIMD alignment and shared advisory read lock.

## Signature

```rust
impl HardenedTensorMap { pub fn load_aligned(path: &Path, offset: usize, count: usize) -> Result<Self, OxideError> }
```

## Type Parameters

- `T: Copy`

## Visibility

- `pub`

## Docstring

Load generic typed tensor slice enforcing >= 64-byte SIMD alignment and shared advisory read lock.

## Source
Lines 281–321 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
