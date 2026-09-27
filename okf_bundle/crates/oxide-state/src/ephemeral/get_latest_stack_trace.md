---
okf_version: "0.2"
type: Function
title: get_latest_stack_trace
description: Retrieve the latest stack trace if present
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/get_latest_stack_trace
language: rust
---

# get_latest_stack_trace

Retrieve the latest stack trace if present

## Signature

```rust
impl EphemeralMemory { pub fn get_latest_stack_trace(&self) -> Option<ActiveStackTrace> }
```

## Visibility

- `pub`

## Docstring

Retrieve the latest stack trace if present

## Source
Lines 133–136 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
