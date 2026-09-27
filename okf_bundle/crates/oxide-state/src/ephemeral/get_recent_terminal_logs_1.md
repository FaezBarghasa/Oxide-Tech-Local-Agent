---
okf_version: "0.2"
type: Function
title: get_recent_terminal_logs
description: Retrieve the most recent terminal logs
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/get_recent_terminal_logs_1
language: rust
---

# get_recent_terminal_logs

Retrieve the most recent terminal logs

## Signature

```rust
pub fn get_recent_terminal_logs(&self, limit: usize) -> Vec<TerminalBufferEntry>
```

## Visibility

- `pub`

## Docstring

Retrieve the most recent terminal logs

## Source
Lines 121–124 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
