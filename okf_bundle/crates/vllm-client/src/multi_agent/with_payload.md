---
okf_version: "0.2"
type: Function
title: with_payload
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/with_payload
language: rust
---

# with_payload

## Signature

```rust
impl AgentMessage { pub fn with_payload(mut self, payload: serde_json::Value) -> Self }
```

## Visibility

- `pub`

## Source
Lines 105–108 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
