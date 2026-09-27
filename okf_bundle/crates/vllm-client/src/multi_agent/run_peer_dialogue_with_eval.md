---
okf_version: "0.2"
type: Function
title: run_peer_dialogue_with_eval
description: "**Peer dialogue with structured consensus evaluation**:"
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/run_peer_dialogue_with_eval
language: rust
---

# run_peer_dialogue_with_eval

**Peer dialogue with structured consensus evaluation**:

## Signature

```rust
impl MultiAgentCoordinator { pub fn run_peer_dialogue_with_eval(
        &self,
        agent_a: &str,
        agent_b: &str,
        opening: &str,
        max_rounds: usize,
        dtx_id: Option<String>,
    ) -> Result<PeerDialogueResult> }
```

## Visibility

- `pub`

## Docstring

**Peer dialogue with structured consensus evaluation**:
Two agents exchange messages while evaluating mutual alignment score,
tracking completed rounds, and detecting early consensus keywords.

## Source
Lines 719–799 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
