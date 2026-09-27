---
okf_version: "0.2"
type: Class
title: TensorMeta
description: "Tensor metadata extracted from a `.safetensors` header."
resource: crates/vllm-client/src/safetensors_loader.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:45:20Z"
concept_id: crates/vllm-client/src/safetensors_loader/TensorMeta
language: rust
---

# TensorMeta

Tensor metadata extracted from a `.safetensors` header.

## Signature

```rust
pub struct TensorMeta
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Tensor metadata extracted from a `.safetensors` header.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `dtype`
- `shape`
- `num_elements`
- `byte_size`

## Source
Lines 11–17 in `crates/vllm-client/src/safetensors_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [safetensors_loader](/crates/vllm-client/src/safetensors_loader.md) |
