---
okf_version: "0.2"
type: Function
title: with_dtx
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/with_dtx
language: rust
---

# with_dtx

## Signature

```rust
impl AgentMessage { pub fn with_dtx(mut self, dtx_id: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 100–103 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
