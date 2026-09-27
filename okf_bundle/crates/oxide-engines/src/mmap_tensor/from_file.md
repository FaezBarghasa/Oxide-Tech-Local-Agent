---
okf_version: "0.2"
type: Function
title: from_file
description: Memory map a model file from disk into virtual address space without copying.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/from_file
language: rust
---

# from_file

Memory map a model file from disk into virtual address space without copying.

## Signature

```rust
impl MmapModel { pub fn from_file(path: P) -> Result<Self, OxideError> }
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Memory map a model file from disk into virtual address space without copying.

## Source
Lines 63–86 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
