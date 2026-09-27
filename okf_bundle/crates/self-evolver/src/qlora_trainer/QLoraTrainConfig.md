---
okf_version: "0.2"
type: Class
title: QLoraTrainConfig
description: Configuration options for dispatching a local QLoRA fine-tuning run.
resource: crates/self-evolver/src/qlora_trainer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:46:02Z"
concept_id: crates/self-evolver/src/qlora_trainer/QLoraTrainConfig
language: rust
---

# QLoraTrainConfig

Configuration options for dispatching a local QLoRA fine-tuning run.

## Signature

```rust
pub struct QLoraTrainConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration options for dispatching a local QLoRA fine-tuning run.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `base_model`
- `lora_rank`
- `lora_alpha`
- `batch_size`
- `learning_rate`
- `target_modules`
- `output_adapter_dir`

## Source
Lines 9–17 in `crates/self-evolver/src/qlora_trainer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qlora_trainer](/crates/self-evolver/src/qlora_trainer.md) |
