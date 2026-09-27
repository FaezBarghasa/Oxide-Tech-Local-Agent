---
okf_version: "0.2"
type: Function
title: fault
description: Transition directly to Fault state from any active state.
resource: crates/oxide-core/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/state_machine/fault_1
language: rust
---

# fault

Transition directly to Fault state from any active state.

## Signature

```rust
pub fn fault(&mut self, code: u32, message: impl Into<String>) -> StateTransition
```

## Visibility

- `pub`

## Docstring

Transition directly to Fault state from any active state.

## Source
Lines 109–123 in `crates/oxide-core/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/oxide-core/src/state_machine.md) |
