---
okf_version: "0.2"
type: Function
title: load_lora_adapter
description: Load or hot-swap a QLoRA adapter onto the base model.
resource: crates/vllm-client/src/candle_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/vllm-client/src/candle_engine/load_lora_adapter
language: rust
---

# load_lora_adapter

Load or hot-swap a QLoRA adapter onto the base model.

## Signature

```rust
impl CandleEngine { pub fn load_lora_adapter(&mut self, lora_path: PathBuf) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Load or hot-swap a QLoRA adapter onto the base model.

## Source
Lines 101–105 in `crates/vllm-client/src/candle_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [candle_engine](/crates/vllm-client/src/candle_engine.md) |
