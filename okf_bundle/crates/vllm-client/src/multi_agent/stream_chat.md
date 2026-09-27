---
okf_version: "0.2"
type: Function
title: stream_chat
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/stream_chat
language: rust
---

# stream_chat

## Signature

```rust
impl EchoProvider { fn stream_chat(&self, _req: ChatRequest) -> anyhow::Result<StreamResult> }
```

## Source
Lines 953–955 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
