---
okf_version: "0.2"
type: Function
title: chat_completion
description: Execute a speculative chat completion.
resource: crates/vllm-client/src/speculative_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/speculative_engine/chat_completion
language: rust
---

# chat_completion

Execute a speculative chat completion.

## Signature

```rust
impl SpeculativeDecodingEngine { pub fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse> }
```

## Visibility

- `pub`

## Docstring

Execute a speculative chat completion.

1. Runs the draft model for speculative tokens.
2. Verifies the generated speculative prefix with the target model.

## Source
Lines 30–73 in `crates/vllm-client/src/speculative_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [speculative_engine](/crates/vllm-client/src/speculative_engine.md) |
