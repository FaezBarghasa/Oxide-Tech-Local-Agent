---
okf_version: "0.2"
type: Function
title: from_pretrained_config
description: Load model architecture configuration from a model directory
resource: crates/model-trainer/src/hf_hub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:47Z"
concept_id: crates/model-trainer/src/hf_hub/from_pretrained_config
language: rust
---

# from_pretrained_config

Load model architecture configuration from a model directory

## Signature

```rust
impl AutoModelForCausalLM { pub fn from_pretrained_config(model_dir: &Path) -> Result<ModelConfig, HfHubError> }
```

## Visibility

- `pub`

## Docstring

Load model architecture configuration from a model directory

## Source
Lines 61–70 in `crates/model-trainer/src/hf_hub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hf_hub](/crates/model-trainer/src/hf_hub.md) |
