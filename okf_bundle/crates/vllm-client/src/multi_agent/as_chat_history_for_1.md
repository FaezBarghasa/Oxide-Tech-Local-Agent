---
okf_version: "0.2"
type: Function
title: as_chat_history_for
description: "Build a simple `Vec<ChatMessage>` for a specific recipient (their view)."
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/as_chat_history_for_1
language: rust
---

# as_chat_history_for

Build a simple `Vec<ChatMessage>` for a specific recipient (their view).

## Signature

```rust
pub fn as_chat_history_for(&self, agent_id: &str) -> Vec<ChatMessage>
```

## Visibility

- `pub`

## Docstring

Build a simple `Vec<ChatMessage>` for a specific recipient (their view).

## Source
Lines 182–199 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
