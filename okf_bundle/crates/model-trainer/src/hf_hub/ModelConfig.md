---
okf_version: "0.2"
type: Class
title: ModelConfig
description: Normalized AutoModel Configuration
resource: crates/model-trainer/src/hf_hub.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:47Z"
concept_id: crates/model-trainer/src/hf_hub/ModelConfig
language: rust
---

# ModelConfig

Normalized AutoModel Configuration

## Signature

```rust
pub struct ModelConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Normalized AutoModel Configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `architectures`
- `dim`
- `n_heads`
- `n_kv_heads`
- `n_layers`
- `intermediate_dim`
- `vocab_size`
- `norm_eps`
- `rope_theta`
- `max_seq_len`

## Source
Lines 22–43 in `crates/model-trainer/src/hf_hub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hf_hub](/crates/model-trainer/src/hf_hub.md) |
