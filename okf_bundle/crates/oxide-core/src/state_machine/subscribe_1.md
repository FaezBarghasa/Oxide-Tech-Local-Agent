---
okf_version: "0.2"
type: Function
title: subscribe
description: Obtain a lock-free watch receiver for downstream state subscribers.
resource: crates/oxide-core/src/state_machine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/state_machine/subscribe_1
language: rust
---

# subscribe

Obtain a lock-free watch receiver for downstream state subscribers.

## Signature

```rust
pub fn subscribe(&self) -> watch::Receiver<AgentState>
```

## Visibility

- `pub`

## Docstring

Obtain a lock-free watch receiver for downstream state subscribers.

## Source
Lines 84–86 in `crates/oxide-core/src/state_machine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state_machine](/crates/oxide-core/src/state_machine.md) |
