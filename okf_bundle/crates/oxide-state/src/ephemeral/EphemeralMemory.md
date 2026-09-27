---
okf_version: "0.2"
type: Class
title: EphemeralMemory
description: Thread-safe Ephemeral Memory Layer for active session states
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/EphemeralMemory
language: rust
---

# EphemeralMemory

Thread-safe Ephemeral Memory Layer for active session states

## Signature

```rust
pub struct EphemeralMemory
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Thread-safe Ephemeral Memory Layer for active session states
[derive(Debug, Clone)]

## Methods

- `max_terminal_entries`
- `terminal_buffer`
- `open_buffers`
- `active_traces`

## Source
Lines 39–44 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
