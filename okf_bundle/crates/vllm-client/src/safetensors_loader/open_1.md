---
okf_version: "0.2"
type: Function
title: open
description: "Open and memory-map a `.safetensors` file."
resource: crates/vllm-client/src/safetensors_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:45:20Z"
concept_id: crates/vllm-client/src/safetensors_loader/open_1
language: rust
---

# open

Open and memory-map a `.safetensors` file.

## Signature

```rust
pub fn open(path: P) -> Result<Self>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Open and memory-map a `.safetensors` file.

## Source
Lines 37–62 in `crates/vllm-client/src/safetensors_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [safetensors_loader](/crates/vllm-client/src/safetensors_loader.md) |
