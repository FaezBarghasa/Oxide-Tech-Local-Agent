---
okf_version: "0.2"
type: Function
title: new
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/new_1
language: rust
---

# new

## Signature

```rust
pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        role: AgentRole,
        content: impl Into<String>,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 77–93 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
