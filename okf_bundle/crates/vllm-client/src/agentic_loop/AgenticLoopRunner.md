---
okf_version: "0.2"
type: Class
title: AgenticLoopRunner
description: "Agentic runner managing multi-turn conversation state, tool calls, and oscillation guardrails."
resource: crates/vllm-client/src/agentic_loop.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/agentic_loop/AgenticLoopRunner
language: rust
---

# AgenticLoopRunner

Agentic runner managing multi-turn conversation state, tool calls, and oscillation guardrails.

## Signature

```rust
pub struct AgenticLoopRunner
```

## Visibility

- `pub`

## Docstring

Agentic runner managing multi-turn conversation state, tool calls, and oscillation guardrails.

## Methods

- `provider`
- `tool_executor`
- `tool_definitions`
- `max_turns`
- `oscillation_threshold`

## Source
Lines 44–50 in `crates/vllm-client/src/agentic_loop.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agentic_loop](/crates/vllm-client/src/agentic_loop.md) |
