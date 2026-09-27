---
okf_version: "0.2"
type: Function
title: deactivate_lora
description: Deactivate / unload the active LoRA adapter.
resource: crates/vllm-client/src/provider.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/provider/deactivate_lora
language: rust
---

# deactivate_lora

Deactivate / unload the active LoRA adapter.

## Signature

```rust
fn deactivate_lora(&self) -> Result<()>
```

## Docstring

Deactivate / unload the active LoRA adapter.

## Source
Lines 232–234 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
