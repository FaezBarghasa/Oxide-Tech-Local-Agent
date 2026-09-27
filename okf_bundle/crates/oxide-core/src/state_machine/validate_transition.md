---
okf_version: "0.2"
type: Function
title: validate_transition
description: Enforce valid state machine transitions according to deterministic lifecycle rules.
resource: crates/oxide-core/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/state_machine/validate_transition
language: rust
---

# validate_transition

Enforce valid state machine transitions according to deterministic lifecycle rules.

## Signature

```rust
impl AgentStateMachine { fn validate_transition(&self, from: &AgentState, to: &AgentState) -> Result<(), OxideError> }
```

## Docstring

Enforce valid state machine transitions according to deterministic lifecycle rules.

## Source
Lines 126–160 in `crates/oxide-core/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/oxide-core/src/state_machine.md) |
