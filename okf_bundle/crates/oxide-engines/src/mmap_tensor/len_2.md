---
okf_version: "0.2"
type: Function
title: len
description: Byte length of the tensor slice.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/len_2
language: rust
---

# len

Byte length of the tensor slice.

## Signature

```rust
impl TensorSlice { pub fn len(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Byte length of the tensor slice.

## Source
Lines 189–191 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
