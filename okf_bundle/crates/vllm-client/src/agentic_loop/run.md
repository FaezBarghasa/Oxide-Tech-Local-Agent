---
okf_version: "0.2"
type: Function
title: run
description: Execute the autonomous agent loop until completion or max turns reached.
resource: crates/vllm-client/src/agentic_loop.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/agentic_loop/run
language: rust
---

# run

Execute the autonomous agent loop until completion or max turns reached.

## Signature

```rust
impl AgenticLoopRunner { pub fn run(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<(String, Vec<ConversationTurn>)> }
```

## Visibility

- `pub`

## Docstring

Execute the autonomous agent loop until completion or max turns reached.

## Source
Lines 69–199 in `crates/vllm-client/src/agentic_loop.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agentic_loop](/crates/vllm-client/src/agentic_loop.md) |
