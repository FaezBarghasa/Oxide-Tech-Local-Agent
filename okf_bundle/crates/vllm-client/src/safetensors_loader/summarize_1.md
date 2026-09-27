---
okf_version: "0.2"
type: Function
title: summarize
description: "Inspect all tensor names, shapes, data types, and total parameters."
resource: crates/vllm-client/src/safetensors_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:45:20Z"
concept_id: crates/vllm-client/src/safetensors_loader/summarize_1
language: rust
---

# summarize

Inspect all tensor names, shapes, data types, and total parameters.

## Signature

```rust
pub fn summarize(&self) -> Result<SafetensorModelSummary>
```

## Visibility

- `pub`

## Docstring

Inspect all tensor names, shapes, data types, and total parameters.

## Source
Lines 70–94 in `crates/vllm-client/src/safetensors_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [safetensors_loader](/crates/vllm-client/src/safetensors_loader.md) |
