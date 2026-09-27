---
okf_version: "0.2"
type: Function
title: thread_history
description: Retrieve the full message history for a thread.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/thread_history
language: rust
---

# thread_history

Retrieve the full message history for a thread.

## Signature

```rust
impl MultiAgentCoordinator { pub fn thread_history(&self, thread_id: &str) -> Option<AgentThread> }
```

## Visibility

- `pub`

## Docstring

Retrieve the full message history for a thread.

## Source
Lines 806–808 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
