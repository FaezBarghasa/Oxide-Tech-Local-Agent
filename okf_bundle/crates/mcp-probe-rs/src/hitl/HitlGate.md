---
okf_version: "0.2"
type: Class
title: HitlGate
description: Manages a simple UNIX‑domain‑socket based Human‑In‑The‑Loop (HITL) confirmation.
resource: crates/mcp-probe-rs/src/hitl.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:34:56Z"
concept_id: crates/mcp-probe-rs/src/hitl/HitlGate
language: rust
---

# HitlGate

Manages a simple UNIX‑domain‑socket based Human‑In‑The‑Loop (HITL) confirmation.

## Signature

```rust
pub struct HitlGate
```

## Decorators

- `derive(Clone)`

## Visibility

- `pub`

## Docstring

Manages a simple UNIX‑domain‑socket based Human‑In‑The‑Loop (HITL) confirmation.

The server creates a listener at the configured `socket_path`. When a gated
operation is requested, `wait_for_confirmation` blocks until an operator writes
`CONFIRM\n` to the socket or the timeout expires.
[derive(Clone)]

## Methods

- `socket_path`
- `timeout_secs`
- `confirm_tx`

## Source
Lines 15–19 in `crates/mcp-probe-rs/src/hitl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hitl](/crates/mcp-probe-rs/src/hitl.md) |
