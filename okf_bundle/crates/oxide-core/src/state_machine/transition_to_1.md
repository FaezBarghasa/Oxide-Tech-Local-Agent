---
okf_version: "0.2"
type: Function
title: transition_to
description: "Attempt a deterministic state transition, validating legal state paths."
resource: crates/oxide-core/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/state_machine/transition_to_1
language: rust
---

# transition_to

Attempt a deterministic state transition, validating legal state paths.

## Signature

```rust
pub fn transition_to(&mut self, next: AgentState) -> Result<StateTransition, OxideError>
```

## Visibility

- `pub`

## Docstring

Attempt a deterministic state transition, validating legal state paths.

## Source
Lines 94–106 in `crates/oxide-core/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/oxide-core/src/state_machine.md) |
