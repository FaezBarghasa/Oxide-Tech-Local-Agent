---
okf_version: "0.2"
type: Function
title: as_ptr
description: Return the raw pointer to tensor data for FFI and CUDA kernels.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/as_ptr
language: rust
---

# as_ptr

Return the raw pointer to tensor data for FFI and CUDA kernels.

## Signature

```rust
impl TensorSlice { pub fn as_ptr(&self) -> *const u8 }
```

## Visibility

- `pub`

## Docstring

Return the raw pointer to tensor data for FFI and CUDA kernels.

## Source
Lines 179–181 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
