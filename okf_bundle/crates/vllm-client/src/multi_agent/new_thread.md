---
okf_version: "0.2"
type: Function
title: new_thread
description: Create a new conversation thread and return its ID.
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/new_thread
language: rust
---

# new_thread

Create a new conversation thread and return its ID.

## Signature

```rust
impl MultiAgentCoordinator { pub fn new_thread(&self) -> String }
```

## Visibility

- `pub`

## Docstring

Create a new conversation thread and return its ID.

## Source
Lines 248–253 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
