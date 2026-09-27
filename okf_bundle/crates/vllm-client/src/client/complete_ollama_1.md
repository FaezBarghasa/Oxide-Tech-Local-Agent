---
okf_version: "0.2"
type: Function
title: complete_ollama
description: "[tracing::instrument(name = \"llm_ollama_request\", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]"
resource: crates/vllm-client/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/vllm-client/src/client/complete_ollama_1
language: rust
---

# complete_ollama

[tracing::instrument(name = "llm_ollama_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]

## Signature

```rust
fn complete_ollama(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error>
```

## Decorators

- `tracing::instrument(name = "llm_ollama_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))`

## Docstring

[tracing::instrument(name = "llm_ollama_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]

## Source
Lines 337–375 in `crates/vllm-client/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/vllm-client/src/client.md) |
