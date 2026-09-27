---
okf_version: "0.2"
type: Function
title: stream_chat
resource: crates/vllm-client/src/llama_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/llama_server/stream_chat
language: rust
---

# stream_chat

## Signature

```rust
impl LlamaServerProvider { fn stream_chat(&self, req: ChatRequest) -> Result<StreamResult> }
```

## Source
Lines 291–371 in `crates/vllm-client/src/llama_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [llama_server](/crates/vllm-client/src/llama_server.md) |
