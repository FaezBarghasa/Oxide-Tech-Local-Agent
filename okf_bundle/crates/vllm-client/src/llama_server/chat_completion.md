---
okf_version: "0.2"
type: Function
title: chat_completion
resource: crates/vllm-client/src/llama_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/llama_server/chat_completion
language: rust
---

# chat_completion

## Signature

```rust
impl LlamaServerProvider { fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse> }
```

## Source
Lines 174–289 in `crates/vllm-client/src/llama_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [llama_server](/crates/vllm-client/src/llama_server.md) |
