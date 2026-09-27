---
okf_version: "0.2"
type: Class
title: ChatRequest
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
concept_id: crates/vllm-client/src/provider/ChatRequest
language: rust
---

# ChatRequest

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct ChatRequest
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `model`
- `messages`
- `temperature`
- `max_tokens`
- `json_mode`
- `history`
- `tools`
- `tool_choice`
- `images`
- `grammar`
- `stop`
- `slot_id`

## Source
Lines 142–169 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
