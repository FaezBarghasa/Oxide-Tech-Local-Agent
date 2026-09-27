---
okf_version: "0.2"
type: Function
title: generate
description: Asynchronously generate tokens leveraging CUDA stream or AMD Zen multi-threaded executor.
resource: crates/vllm-client/src/candle_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/vllm-client/src/candle_engine/generate
language: rust
---

# generate

Asynchronously generate tokens leveraging CUDA stream or AMD Zen multi-threaded executor.

## Signature

```rust
impl CandleEngine { pub fn generate(&self, prompt: &str) -> Result<String> }
```

## Visibility

- `pub`

## Docstring

Asynchronously generate tokens leveraging CUDA stream or AMD Zen multi-threaded executor.

## Source
Lines 114–141 in `crates/vllm-client/src/candle_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [candle_engine](/crates/vllm-client/src/candle_engine.md) |
