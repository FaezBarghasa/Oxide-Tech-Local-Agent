---
okf_version: "0.2"
type: Function
title: wait_for_confirmation
description: "Blocks until an operator client sends a line containing \"CONFIRM\" or the timeout"
resource: crates/mcp-probe-rs/src/hitl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:34:56Z"
concept_id: crates/mcp-probe-rs/src/hitl/wait_for_confirmation
language: rust
---

# wait_for_confirmation

Blocks until an operator client sends a line containing "CONFIRM" or the timeout

## Signature

```rust
impl HitlGate { pub fn wait_for_confirmation(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Blocks until an operator client sends a line containing "CONFIRM" or the timeout
expires. Returns `true` on confirmation, `false` otherwise.

## Source
Lines 75–90 in `crates/mcp-probe-rs/src/hitl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hitl](/crates/mcp-probe-rs/src/hitl.md) |
