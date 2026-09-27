---
okf_version: "0.2"
type: Class
title: LoraAdapterConfig
description: Metadata and configuration for a dynamic LoRA adapter
resource: crates/router/src/lora_router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/lora_router/LoraAdapterConfig
language: rust
---

# LoraAdapterConfig

Metadata and configuration for a dynamic LoRA adapter

## Signature

```rust
pub struct LoraAdapterConfig
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

Metadata and configuration for a dynamic LoRA adapter
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `adapter_type`
- `adapter_name`
- `rank`
- `alpha`
- `path`
- `target_modules`
- `domain_keywords`

## Source
Lines 17–25 in `crates/router/src/lora_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lora_router](/crates/router/src/lora_router.md) |
