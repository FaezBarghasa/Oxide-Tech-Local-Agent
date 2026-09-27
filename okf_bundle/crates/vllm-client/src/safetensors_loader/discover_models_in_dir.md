---
okf_version: "0.2"
type: Function
title: discover_models_in_dir
description: "Discover and list all `.safetensors` files in a given directory (e.g. HuggingFace snapshot)."
resource: crates/vllm-client/src/safetensors_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:45:20Z"
concept_id: crates/vllm-client/src/safetensors_loader/discover_models_in_dir
language: rust
---

# discover_models_in_dir

Discover and list all `.safetensors` files in a given directory (e.g. HuggingFace snapshot).

## Signature

```rust
impl SafetensorModelLoader { pub fn discover_models_in_dir(dir: P) -> Result<Vec<PathBuf>> }
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Discover and list all `.safetensors` files in a given directory (e.g. HuggingFace snapshot).

## Source
Lines 110–123 in `crates/vllm-client/src/safetensors_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [safetensors_loader](/crates/vllm-client/src/safetensors_loader.md) |
