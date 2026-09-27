---
okf_version: "0.2"
type: Function
title: current
description: Read the current state directly.
resource: crates/oxide-core/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/state_machine/current
language: rust
---

# current

Read the current state directly.

## Signature

```rust
impl AgentStateMachine { pub fn current(&self) -> &AgentState }
```

## Visibility

- `pub`

## Docstring

Read the current state directly.

## Source
Lines 79–81 in `crates/oxide-core/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/oxide-core/src/state_machine.md) |
