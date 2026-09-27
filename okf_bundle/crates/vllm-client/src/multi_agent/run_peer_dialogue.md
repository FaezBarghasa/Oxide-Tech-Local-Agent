---
okf_version: "0.2"
type: Function
title: run_peer_dialogue
description: "**Peer dialogue**: Two agents exchange messages for up to `max_rounds`"
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/run_peer_dialogue
language: rust
---

# run_peer_dialogue

**Peer dialogue**: Two agents exchange messages for up to `max_rounds`

## Signature

```rust
impl MultiAgentCoordinator { pub fn run_peer_dialogue(
        &self,
        agent_a: &str,
        agent_b: &str,
        opening: &str,
        max_rounds: usize,
    ) -> Result<(String, String)> }
```

## Visibility

- `pub`

## Docstring

**Peer dialogue**: Two agents exchange messages for up to `max_rounds`
rounds. Useful for debate, critique, or co-refinement flows.

Returns the last reply and the thread ID.

## Source
Lines 703–714 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
