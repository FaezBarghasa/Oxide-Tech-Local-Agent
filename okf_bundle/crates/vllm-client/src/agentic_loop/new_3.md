---
okf_version: "0.2"
type: Function
title: new
resource: crates/vllm-client/src/agentic_loop.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/agentic_loop/new_3
language: rust
---

# new

## Signature

```rust
pub fn new(
        provider: Arc<dyn InferenceProvider>,
        tool_executor: Arc<dyn ToolExecutor>,
        tool_definitions: Vec<ToolDefinition>,
        max_turns: usize,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 53–66 in `crates/vllm-client/src/agentic_loop.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agentic_loop](/crates/vllm-client/src/agentic_loop.md) |
