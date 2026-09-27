---
okf_version: "0.2"
type: Function
title: get_tensor_slice
description: Extract a zero-copy tensor slice with strict boundary and alignment checks.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/get_tensor_slice
language: rust
---

# get_tensor_slice

Extract a zero-copy tensor slice with strict boundary and alignment checks.

## Signature

```rust
impl MmapModel { pub fn get_tensor_slice(
        &self,
        offset: usize,
        size_bytes: usize,
        alignment: usize,
    ) -> Result<TensorSlice, OxideError> }
```

## Visibility

- `pub`

## Docstring

Extract a zero-copy tensor slice with strict boundary and alignment checks.

## Source
Lines 128–162 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
