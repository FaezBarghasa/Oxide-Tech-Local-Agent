---
okf_version: "0.2"
type: Class
title: AgentThread
description: Running record of messages exchanged in a multi-agent conversation.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/AgentThread
language: rust
---

# AgentThread

Running record of messages exchanged in a multi-agent conversation.

## Signature

```rust
pub struct AgentThread
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Running record of messages exchanged in a multi-agent conversation.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `messages`
- `final_answer`

## Source
Lines 162–166 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
