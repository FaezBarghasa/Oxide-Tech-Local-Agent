---
okf_version: "0.2"
type: Class
title: StateTransition
description: Recorded state transition with timestamp.
resource: crates/oxide-core/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/state_machine/StateTransition
language: rust
---

# StateTransition

Recorded state transition with timestamp.

## Signature

```rust
pub struct StateTransition
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Recorded state transition with timestamp.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `from`
- `to`
- `timestamp_ms`

## Source
Lines 45–49 in `crates/oxide-core/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/oxide-core/src/state_machine.md) |
