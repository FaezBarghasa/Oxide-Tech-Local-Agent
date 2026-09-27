---
okf_version: "0.2"
type: Function
title: push_stack_trace
description: Capture an active panic or runtime error stack trace
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/push_stack_trace
language: rust
---

# push_stack_trace

Capture an active panic or runtime error stack trace

## Signature

```rust
impl EphemeralMemory { pub fn push_stack_trace(&self, error_type: &str, message: &str, frames: Vec<String>) }
```

## Visibility

- `pub`

## Docstring

Capture an active panic or runtime error stack trace

## Source
Lines 106–118 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
