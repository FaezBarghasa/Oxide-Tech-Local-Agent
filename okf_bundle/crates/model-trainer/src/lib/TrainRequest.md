---
okf_version: "0.2"
type: Class
title: TrainRequest
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/model-trainer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:59:29Z"
concept_id: crates/model-trainer/src/lib/TrainRequest
language: rust
---

# TrainRequest

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct TrainRequest
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `task_id`
- `kind`
- `base_model`
- `dataset_path`
- `output_dir`
- `epochs`
- `learning_rate`
- `batch_size`
- `lora_rank`
- `lora_alpha`
- `qlora_config`

## Source
Lines 93–106 in `crates/model-trainer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/model-trainer/src/lib.md) |
