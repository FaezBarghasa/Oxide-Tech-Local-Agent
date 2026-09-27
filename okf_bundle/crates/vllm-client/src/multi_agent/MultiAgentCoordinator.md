---
okf_version: "0.2"
type: Class
title: MultiAgentCoordinator
description: Central coordinator for multi-agent message passing and orchestration.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/MultiAgentCoordinator
language: rust
---

# MultiAgentCoordinator

Central coordinator for multi-agent message passing and orchestration.

## Signature

```rust
pub struct MultiAgentCoordinator
```

## Visibility

- `pub`

## Docstring

Central coordinator for multi-agent message passing and orchestration.

# Topology patterns supported

- **Supervisor → Worker(s) → Verifier**: Supervisor decomposes task, Workers
execute in parallel, Verifier scores and requests revisions.
- **Chain**: A → B → C → final, each agent's output becomes the next input.
- **Peer dialogue**: Any two agents exchange messages in a shared thread.
- **Broadcast**: One agent sends to all others simultaneously.

## Methods

- `agents`
- `threads`
- `event_bus`

## Source
Lines 213–217 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
