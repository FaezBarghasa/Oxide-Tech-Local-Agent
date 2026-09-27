---
okf_version: "0.2"
type: Class
title: TerminalBufferEntry
description: An entry in the active terminal session ephemeral ring buffer
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/TerminalBufferEntry
language: rust
---

# TerminalBufferEntry

An entry in the active terminal session ephemeral ring buffer

## Signature

```rust
pub struct TerminalBufferEntry
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

An entry in the active terminal session ephemeral ring buffer
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `session_id`
- `command`
- `output_snippet`
- `exit_code`
- `timestamp`

## Source
Lines 8–14 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
