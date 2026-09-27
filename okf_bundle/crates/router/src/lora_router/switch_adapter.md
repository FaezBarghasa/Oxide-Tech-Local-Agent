---
okf_version: "0.2"
type: Function
title: switch_adapter
description: Dispatch dynamic LoRA hot-swap request to SGLang server
resource: crates/router/src/lora_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/lora_router/switch_adapter
language: rust
---

# switch_adapter

Dispatch dynamic LoRA hot-swap request to SGLang server

## Signature

```rust
impl DynamicLoraRouter { pub fn switch_adapter(&mut self, new_adapter: LoraAdapterType) -> String }
```

## Visibility

- `pub`

## Docstring

Dispatch dynamic LoRA hot-swap request to SGLang server

## Source
Lines 175–182 in `crates/router/src/lora_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lora_router](/crates/router/src/lora_router.md) |
