---
okf_version: "0.2"
type: Class
title: ChatResponse
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/vllm-client/src/provider.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/provider/ChatResponse
language: rust
---

# ChatResponse

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct ChatResponse
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `content`
- `prompt_tokens`
- `completion_tokens`
- `finish_reason`
- `tool_calls`
- `latency_ms`
- `slot_id`

## Source
Lines 172–185 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
