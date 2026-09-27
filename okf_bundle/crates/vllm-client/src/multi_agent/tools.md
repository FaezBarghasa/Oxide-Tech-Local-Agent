---
okf_version: "0.2"
type: Function
title: tools
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/tools
language: rust
---

# tools

## Signature

```rust
impl AgentBuilder { pub fn tools(mut self, tools: Vec<ToolDefinition>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 877–880 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
