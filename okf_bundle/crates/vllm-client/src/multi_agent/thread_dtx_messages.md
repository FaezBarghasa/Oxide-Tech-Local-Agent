---
okf_version: "0.2"
type: Function
title: thread_dtx_messages
description: Retrieve all messages associated with a specific DTX transaction ID across a thread.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/thread_dtx_messages
language: rust
---

# thread_dtx_messages

Retrieve all messages associated with a specific DTX transaction ID across a thread.

## Signature

```rust
impl MultiAgentCoordinator { pub fn thread_dtx_messages(&self, thread_id: &str, dtx_id: &str) -> Vec<AgentMessage> }
```

## Visibility

- `pub`

## Docstring

Retrieve all messages associated with a specific DTX transaction ID across a thread.

## Source
Lines 811–823 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
