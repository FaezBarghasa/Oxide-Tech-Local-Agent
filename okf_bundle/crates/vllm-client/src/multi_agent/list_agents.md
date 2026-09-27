---
okf_version: "0.2"
type: Function
title: list_agents
description: List all registered agents (id → role).
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/list_agents
language: rust
---

# list_agents

List all registered agents (id → role).

## Signature

```rust
impl MultiAgentCoordinator { pub fn list_agents(&self) -> Vec<(String, AgentRole)> }
```

## Visibility

- `pub`

## Docstring

List all registered agents (id → role).

## Source
Lines 826–833 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
