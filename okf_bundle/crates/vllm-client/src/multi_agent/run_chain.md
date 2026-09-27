---
okf_version: "0.2"
type: Function
title: run_chain
description: "**Sequential chain**: Pass the task through agents A → B → C → …,"
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/run_chain
language: rust
---

# run_chain

**Sequential chain**: Pass the task through agents A → B → C → …,

## Signature

```rust
impl MultiAgentCoordinator { pub fn run_chain(
        &self,
        agent_ids: &[&str],
        initial_prompt: &str,
    ) -> Result<(String, String)> }
```

## Visibility

- `pub`

## Docstring

**Sequential chain**: Pass the task through agents A → B → C → …,
feeding each agent's output as the next agent's input.

Returns the final agent's output and the full thread ID.

## Source
Lines 456–463 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
