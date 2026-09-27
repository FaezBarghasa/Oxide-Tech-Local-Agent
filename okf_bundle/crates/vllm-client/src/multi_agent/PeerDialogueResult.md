---
okf_version: "0.2"
type: Class
title: PeerDialogueResult
description: "Result of a peer dialogue execution, including consensus evaluation."
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/PeerDialogueResult
language: rust
---

# PeerDialogueResult

Result of a peer dialogue execution, including consensus evaluation.

## Signature

```rust
pub struct PeerDialogueResult
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Result of a peer dialogue execution, including consensus evaluation.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `final_content`
- `thread_id`
- `rounds_completed`
- `consensus_reached`
- `consensus_score`
- `dtx_id`

## Source
Lines 44–51 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
