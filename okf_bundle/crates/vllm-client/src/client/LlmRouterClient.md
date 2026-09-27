---
okf_version: "0.2"
type: Class
title: LlmRouterClient
description: "A unified LLM client that dispatches to Groq, Mistral, Ollama, or a"
resource: crates/vllm-client/src/client.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/vllm-client/src/client/LlmRouterClient
language: rust
---

# LlmRouterClient

A unified LLM client that dispatches to Groq, Mistral, Ollama, or a

## Signature

```rust
pub struct LlmRouterClient
```

## Visibility

- `pub`

## Docstring

A unified LLM client that dispatches to Groq, Mistral, Ollama, or a
self-hosted vLLM instance based on the configured provider.

API keys are resolved from environment variables:
- `GROQ_API_KEY`
- `MISTRAL_API_KEY`
- `VLLM_API_KEY` (optional, for authenticated vLLM deployments)

Ollama never requires a key.

## Methods

- `provider`
- `model`
- `base_url`
- `http`

## Source
Lines 159–164 in `crates/vllm-client/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/vllm-client/src/client.md) |
