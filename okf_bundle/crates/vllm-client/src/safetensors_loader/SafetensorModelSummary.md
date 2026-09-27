---
okf_version: "0.2"
type: Class
title: SafetensorModelSummary
description: "Model info summary for loaded or inspected `.safetensors` model files."
resource: crates/vllm-client/src/safetensors_loader.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:45:20Z"
concept_id: crates/vllm-client/src/safetensors_loader/SafetensorModelSummary
language: rust
---

# SafetensorModelSummary

Model info summary for loaded or inspected `.safetensors` model files.

## Signature

```rust
pub struct SafetensorModelSummary
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Model info summary for loaded or inspected `.safetensors` model files.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `path`
- `total_tensors`
- `total_parameters`
- `total_bytes`
- `tensor_names`

## Source
Lines 21–27 in `crates/vllm-client/src/safetensors_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [safetensors_loader](/crates/vllm-client/src/safetensors_loader.md) |
