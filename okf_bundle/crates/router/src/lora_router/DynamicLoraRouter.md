---
okf_version: "0.2"
type: Class
title: DynamicLoraRouter
description: "Dynamic LoRA Router: Selects task-specific LoRA weights using keyword frequency scoring and fallback classifiers."
resource: crates/router/src/lora_router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/lora_router/DynamicLoraRouter
language: rust
---

# DynamicLoraRouter

Dynamic LoRA Router: Selects task-specific LoRA weights using keyword frequency scoring and fallback classifiers.

## Signature

```rust
pub struct DynamicLoraRouter
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Dynamic LoRA Router: Selects task-specific LoRA weights using keyword frequency scoring and fallback classifiers.
[derive(Debug, Clone)]

## Methods

- `active_adapter`
- `adapters`

## Source
Lines 29–32 in `crates/router/src/lora_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lora_router](/crates/router/src/lora_router.md) |
