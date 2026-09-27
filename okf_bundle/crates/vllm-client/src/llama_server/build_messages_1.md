---
okf_version: "0.2"
type: Function
title: build_messages
description: "Convert `ConversationTurn` history + current `ChatRequest` into the"
resource: crates/vllm-client/src/llama_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/llama_server/build_messages_1
language: rust
---

# build_messages

Convert `ConversationTurn` history + current `ChatRequest` into the

## Signature

```rust
fn build_messages(req: &ChatRequest) -> Vec<Value>
```

## Docstring

Convert `ConversationTurn` history + current `ChatRequest` into the
OpenAI messages array that llama-server expects.

## Source
Lines 45–100 in `crates/vllm-client/src/llama_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [llama_server](/crates/vllm-client/src/llama_server.md) |
