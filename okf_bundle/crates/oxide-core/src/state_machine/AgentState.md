---
okf_version: "0.2"
type: Class
title: AgentState
description: Comprehensive state enum for the agent loop lifecycle.
resource: crates/oxide-core/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/state_machine/AgentState
language: rust
---

# AgentState

Comprehensive state enum for the agent loop lifecycle.

## Signature

```rust
pub enum AgentState
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)`
- `serde(tag = "type", content = "payload")`

## Visibility

- `pub`

## Docstring

Comprehensive state enum for the agent loop lifecycle.
[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
[serde(tag = "type", content = "payload")]

## Methods

- `context_len`
- `tool_name`
- `call_id`
- `tokens_emitted`
- `reason`
- `code`
- `message`

## Source
Lines 11–24 in `crates/oxide-core/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/oxide-core/src/state_machine.md) |
