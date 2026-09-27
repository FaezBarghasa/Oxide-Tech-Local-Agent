---
okf_version: "0.2"
type: Function
title: as_slice
description: Access mapped data as an immutable f32 slice.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/as_slice_2
language: rust
---

# as_slice

Access mapped data as an immutable f32 slice.

## Signature

```rust
impl AlignedTensorMap { pub fn as_slice(&self) -> &[f32] }
```

## Visibility

- `pub`

## Docstring

Access mapped data as an immutable f32 slice.

## Source
Lines 253–258 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
