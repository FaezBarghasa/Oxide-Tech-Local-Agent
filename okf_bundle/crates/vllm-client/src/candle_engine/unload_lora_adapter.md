---
okf_version: "0.2"
type: Function
title: unload_lora_adapter
description: "Unload active LoRA adapter, reverting to pure base weights."
resource: crates/vllm-client/src/candle_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/vllm-client/src/candle_engine/unload_lora_adapter
language: rust
---

# unload_lora_adapter

Unload active LoRA adapter, reverting to pure base weights.

## Signature

```rust
impl CandleEngine { pub fn unload_lora_adapter(&mut self) }
```

## Visibility

- `pub`

## Docstring

Unload active LoRA adapter, reverting to pure base weights.

## Source
Lines 108–111 in `crates/vllm-client/src/candle_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [candle_engine](/crates/vllm-client/src/candle_engine.md) |
