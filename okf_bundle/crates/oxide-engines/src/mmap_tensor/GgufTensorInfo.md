---
okf_version: "0.2"
type: Class
title: GgufTensorInfo
description: Metadata describing an individual tensor inside an IMatrix / GGUF model file.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/GgufTensorInfo
language: rust
---

# GgufTensorInfo

Metadata describing an individual tensor inside an IMatrix / GGUF model file.

## Signature

```rust
pub struct GgufTensorInfo
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Metadata describing an individual tensor inside an IMatrix / GGUF model file.
[derive(Debug, Clone)]

## Methods

- `name`
- `tensor_type`
- `shape`
- `offset`
- `size_bytes`

## Source
Lines 34–40 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
