---
okf_version: "0.2"
type: Function
title: complete
description: Run a single non-agentic completion (no tool loop).
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/complete_1
language: rust
---

# complete

Run a single non-agentic completion (no tool loop).

## Signature

```rust
pub fn complete(&self, messages: Vec<ChatMessage>) -> Result<String>
```

## Visibility

- `pub`

## Docstring

Run a single non-agentic completion (no tool loop).

## Source
Lines 138–155 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
