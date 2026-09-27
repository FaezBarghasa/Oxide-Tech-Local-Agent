---
okf_version: "0.2"
type: Class
title: InferenceProvider
description: "Pluggable inference provider abstraction across Ollama, SGLang,"
resource: crates/vllm-client/src/provider.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/provider/InferenceProvider
language: rust
---

# InferenceProvider

Pluggable inference provider abstraction across Ollama, SGLang,

## Signature

```rust
pub trait InferenceProvider
```

## Decorators

- `async_trait`

## Visibility

- `pub`

## Docstring

Pluggable inference provider abstraction across Ollama, SGLang,
llama-server, llama.cpp, Candle, vLLM and all OpenAI-compatible endpoints.
[async_trait]

## Source
Lines 217–240 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
