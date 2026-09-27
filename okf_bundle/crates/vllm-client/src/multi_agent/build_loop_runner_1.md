---
okf_version: "0.2"
type: Function
title: build_loop_runner
description: "Build an `AgenticLoopRunner` for this agent on demand."
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/build_loop_runner_1
language: rust
---

# build_loop_runner

Build an `AgenticLoopRunner` for this agent on demand.

## Signature

```rust
pub fn build_loop_runner(&self) -> Option<AgenticLoopRunner>
```

## Visibility

- `pub`

## Docstring

Build an `AgenticLoopRunner` for this agent on demand.

## Source
Lines 126–135 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
