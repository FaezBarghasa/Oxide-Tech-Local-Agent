---
okf_version: "0.2"
type: Function
title: register
description: Register an agent with the coordinator.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/register
language: rust
---

# register

Register an agent with the coordinator.

## Signature

```rust
impl MultiAgentCoordinator { pub fn register(&self, record: AgentRecord) }
```

## Visibility

- `pub`

## Docstring

Register an agent with the coordinator.

## Source
Lines 241–245 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
