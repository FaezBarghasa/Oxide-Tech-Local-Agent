---
okf_version: "0.2"
type: Function
title: emit
description: "Non-blocking send: if queue is full, increments dropped_count and returns immediately"
resource: crates/oxide-security/src/bounded_outbox.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/bounded_outbox/emit_1
language: rust
---

# emit

Non-blocking send: if queue is full, increments dropped_count and returns immediately

## Signature

```rust
pub fn emit(&self, level: &str, target: &str, message: &str) -> bool
```

## Visibility

- `pub`

## Docstring

Non-blocking send: if queue is full, increments dropped_count and returns immediately

## Source
Lines 66–81 in `crates/oxide-security/src/bounded_outbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bounded_outbox](/crates/oxide-security/src/bounded_outbox.md) |
