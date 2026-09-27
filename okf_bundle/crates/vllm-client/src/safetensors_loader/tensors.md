---
okf_version: "0.2"
type: Function
title: tensors
description: "Retrieve the parsed `SafeTensors` view over the memory map."
resource: crates/vllm-client/src/safetensors_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:45:20Z"
concept_id: crates/vllm-client/src/safetensors_loader/tensors
language: rust
---

# tensors

Retrieve the parsed `SafeTensors` view over the memory map.

## Signature

```rust
impl SafetensorModelLoader { pub fn tensors(&self) -> Result<SafeTensors<'_>, SafeTensorError> }
```

## Visibility

- `pub`

## Docstring

Retrieve the parsed `SafeTensors` view over the memory map.

## Source
Lines 65–67 in `crates/vllm-client/src/safetensors_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [safetensors_loader](/crates/vllm-client/src/safetensors_loader.md) |
