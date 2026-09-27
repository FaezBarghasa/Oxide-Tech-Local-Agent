---
okf_version: "0.2"
type: Function
title: record_terminal_output
description: Record a command execution into the ring buffer
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/record_terminal_output_1
language: rust
---

# record_terminal_output

Record a command execution into the ring buffer

## Signature

```rust
pub fn record_terminal_output(
        &self,
        session_id: &str,
        command: &str,
        output_snippet: &str,
        exit_code: Option<i32>,
    )
```

## Visibility

- `pub`

## Docstring

Record a command execution into the ring buffer

## Source
Lines 57–75 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
