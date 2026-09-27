---
okf_version: "0.2"
type: Function
title: get_tensor_data
description: "Extract a specific tensor's raw byte slice by name."
resource: crates/vllm-client/src/safetensors_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:45:20Z"
concept_id: crates/vllm-client/src/safetensors_loader/get_tensor_data
language: rust
---

# get_tensor_data

Extract a specific tensor's raw byte slice by name.

## Signature

```rust
impl SafetensorModelLoader { pub fn get_tensor_data(&self, tensor_name: &str) -> Result<&[u8]> }
```

## Visibility

- `pub`

## Docstring

Extract a specific tensor's raw byte slice by name.

## Source
Lines 97–107 in `crates/vllm-client/src/safetensors_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [safetensors_loader](/crates/vllm-client/src/safetensors_loader.md) |
