---
okf_version: "0.2"
type: Class
title: ComputeDevice
description: Supported hardware accelerator compute device for native model execution.
resource: crates/vllm-client/src/candle_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/vllm-client/src/candle_engine/ComputeDevice
language: rust
---

# ComputeDevice

Supported hardware accelerator compute device for native model execution.

## Signature

```rust
pub enum ComputeDevice
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Supported hardware accelerator compute device for native model execution.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `gpu_id`
- `flash_attention`
- `threads`
- `use_avx512`
- `threads`

## Source
Lines 8–18 in `crates/vllm-client/src/candle_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [candle_engine](/crates/vllm-client/src/candle_engine.md) |
