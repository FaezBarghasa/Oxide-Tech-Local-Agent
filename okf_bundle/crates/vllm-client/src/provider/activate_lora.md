---
okf_version: "0.2"
type: Function
title: activate_lora
description: Activate a LoRA adapter by name/path. No-op if not supported.
resource: crates/vllm-client/src/provider.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/provider/activate_lora
language: rust
---

# activate_lora

Activate a LoRA adapter by name/path. No-op if not supported.

## Signature

```rust
fn activate_lora(&self, _adapter_id: &str) -> Result<()>
```

## Docstring

Activate a LoRA adapter by name/path. No-op if not supported.

## Source
Lines 227–229 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
