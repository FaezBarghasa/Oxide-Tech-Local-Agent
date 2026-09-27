---
okf_version: "0.2"
type: Function
title: export_gguf_lora_container
description: Convert LoRA weights to GGUF adapter format for direct mounting in llama.cpp / Ollama
resource: crates/model-trainer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:59:29Z"
concept_id: crates/model-trainer/src/lib/export_gguf_lora_container
language: rust
---

# export_gguf_lora_container

Convert LoRA weights to GGUF adapter format for direct mounting in llama.cpp / Ollama

## Signature

```rust
impl QLoraTrainer { pub fn export_gguf_lora_container(
        adapter_id: Uuid,
        base_model: &str,
        output_dir: &std::path::Path,
        rank: u32,
        alpha: u32,
        quant_type: GgufQuantType,
    ) -> Result<PathBuf, TrainerError> }
```

## Visibility

- `pub`

## Docstring

Convert LoRA weights to GGUF adapter format for direct mounting in llama.cpp / Ollama

## Source
Lines 303–356 in `crates/model-trainer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/model-trainer/src/lib.md) |
