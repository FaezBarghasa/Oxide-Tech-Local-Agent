---
okf_version: "0.2"
type: Function
title: select_adapter_for_prompt
description: Predict the optimal LoRA adapter using weighted domain keyword ranking
resource: crates/router/src/lora_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/lora_router/select_adapter_for_prompt
language: rust
---

# select_adapter_for_prompt

Predict the optimal LoRA adapter using weighted domain keyword ranking

## Signature

```rust
impl DynamicLoraRouter { pub fn select_adapter_for_prompt(&self, prompt: &str) -> LoraAdapterType }
```

## Visibility

- `pub`

## Docstring

Predict the optimal LoRA adapter using weighted domain keyword ranking

## Source
Lines 153–172 in `crates/router/src/lora_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lora_router](/crates/router/src/lora_router.md) |
