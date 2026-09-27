---
okf_version: "0.2"
type: Function
title: broadcast
description: Broadcast a message to all agents and collect their replies in parallel.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/broadcast_1
language: rust
---

# broadcast

Broadcast a message to all agents and collect their replies in parallel.

## Signature

```rust
pub fn broadcast(
        &self,
        thread_id: &str,
        from_agent: &str,
        from_role: AgentRole,
        content: &str,
    ) -> Result<Vec<AgentMessage>>
```

## Visibility

- `pub`

## Docstring

Broadcast a message to all agents and collect their replies in parallel.

## Source
Lines 348–446 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
