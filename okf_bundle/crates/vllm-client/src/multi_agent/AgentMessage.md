---
okf_version: "0.2"
type: Class
title: AgentMessage
description: A message sent from one agent to another within the coordinator.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/AgentMessage
language: rust
---

# AgentMessage

A message sent from one agent to another within the coordinator.

## Signature

```rust
pub struct AgentMessage
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A message sent from one agent to another within the coordinator.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `from_agent`
- `to_agent`
- `sender_role`
- `content`
- `thread_id`
- `dtx_id`
- `payload`

## Source
Lines 57–74 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
