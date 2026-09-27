---
okf_version: "0.2"
type: Function
title: spawn_listener
description: "Starts the background listener. It runs forever, accepting connections"
resource: crates/mcp-probe-rs/src/hitl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:34:56Z"
concept_id: crates/mcp-probe-rs/src/hitl/spawn_listener
language: rust
---

# spawn_listener

Starts the background listener. It runs forever, accepting connections

## Signature

```rust
impl HitlGate { pub fn spawn_listener(&self) }
```

## Visibility

- `pub`

## Docstring

Starts the background listener. It runs forever, accepting connections
and parsing confirmation messages.

## Source
Lines 33–71 in `crates/mcp-probe-rs/src/hitl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hitl](/crates/mcp-probe-rs/src/hitl.md) |
