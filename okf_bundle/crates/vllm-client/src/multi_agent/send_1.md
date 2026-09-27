---
okf_version: "0.2"
type: Function
title: send
description: Send a message to a specific agent and get its reply.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/send_1
language: rust
---

# send

Send a message to a specific agent and get its reply.

## Signature

```rust
pub fn send(&self, thread_id: &str, message: AgentMessage) -> Result<AgentMessage>
```

## Visibility

- `pub`

## Docstring

Send a message to a specific agent and get its reply.

The full thread history visible to the receiving agent is included.

## Source
Lines 262–345 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
