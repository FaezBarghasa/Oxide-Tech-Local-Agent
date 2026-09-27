---
okf_version: "0.2"
type: Function
title: tool_executor
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/tool_executor_1
language: rust
---

# tool_executor

## Signature

```rust
pub fn tool_executor(mut self, executor: Arc<dyn ToolExecutor>) -> Self
```

## Visibility

- `pub`

## Source
Lines 872–875 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
