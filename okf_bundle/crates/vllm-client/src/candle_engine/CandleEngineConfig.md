---
okf_version: "0.2"
type: Class
title: CandleEngineConfig
description: Configuration for the Candle native inference runtime with hardware acceleration.
resource: crates/vllm-client/src/candle_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/vllm-client/src/candle_engine/CandleEngineConfig
language: rust
---

# CandleEngineConfig

Configuration for the Candle native inference runtime with hardware acceleration.

## Signature

```rust
pub struct CandleEngineConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration for the Candle native inference runtime with hardware acceleration.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `model_id_or_path`
- `temperature`
- `top_p`
- `max_tokens`
- `active_lora_path`
- `compute_device`

## Source
Lines 64–71 in `crates/vllm-client/src/candle_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [candle_engine](/crates/vllm-client/src/candle_engine.md) |
