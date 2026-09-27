---
okf_version: "0.2"
type: Function
title: ollama
description: Build an explicit Ollama client (convenience constructor).
resource: crates/vllm-client/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/vllm-client/src/client/ollama_1
language: rust
---

# ollama

Build an explicit Ollama client (convenience constructor).

## Signature

```rust
pub fn ollama(base_url: &str, model: &str) -> Self
```

## Visibility

- `pub`

## Docstring

Build an explicit Ollama client (convenience constructor).

## Source
Lines 182–192 in `crates/vllm-client/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/vllm-client/src/client.md) |
