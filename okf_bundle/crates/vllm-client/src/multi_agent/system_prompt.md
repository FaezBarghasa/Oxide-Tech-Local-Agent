---
okf_version: "0.2"
type: Function
title: system_prompt
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/system_prompt
language: rust
---

# system_prompt

## Signature

```rust
impl AgentBuilder { pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 862–865 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
