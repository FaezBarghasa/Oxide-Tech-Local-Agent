---
okf_version: "0.2"
type: Function
title: stream_chat
resource: crates/vllm-client/src/sglang_provider.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:58:19Z"
concept_id: crates/vllm-client/src/sglang_provider/stream_chat
language: rust
---

# stream_chat

## Signature

```rust
impl SglangProvider { fn stream_chat(&self, req: ChatRequest) -> Result<StreamResult> }
```

## Source
Lines 148–219 in `crates/vllm-client/src/sglang_provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sglang_provider](/crates/vllm-client/src/sglang_provider.md) |
