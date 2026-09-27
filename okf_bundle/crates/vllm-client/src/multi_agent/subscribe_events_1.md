---
okf_version: "0.2"
type: Function
title: subscribe_events
description: Subscribe to the live EventBus stream of all inter-agent messages.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/subscribe_events_1
language: rust
---

# subscribe_events

Subscribe to the live EventBus stream of all inter-agent messages.

## Signature

```rust
pub fn subscribe_events(&self) -> tokio::sync::broadcast::Receiver<AgentMessage>
```

## Visibility

- `pub`

## Docstring

Subscribe to the live EventBus stream of all inter-agent messages.

## Source
Lines 236–238 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
