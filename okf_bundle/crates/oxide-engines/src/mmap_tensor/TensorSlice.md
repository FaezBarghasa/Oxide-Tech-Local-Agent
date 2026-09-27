---
okf_version: "0.2"
type: Class
title: TensorSlice
description: "A zero-copy slice of memory-mapped model weights tied to the underlying `Mmap` lifetime."
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/TensorSlice
language: rust
---

# TensorSlice

A zero-copy slice of memory-mapped model weights tied to the underlying `Mmap` lifetime.

## Signature

```rust
pub struct TensorSlice
```

## Decorators

- `derive(Clone)`

## Visibility

- `pub`

## Docstring

A zero-copy slice of memory-mapped model weights tied to the underlying `Mmap` lifetime.
[derive(Clone)]

## Methods

- `_owner`
- `ptr`
- `len`

## Source
Lines 167–171 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
