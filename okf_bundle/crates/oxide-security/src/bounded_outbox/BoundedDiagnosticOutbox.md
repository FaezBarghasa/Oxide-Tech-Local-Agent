---
okf_version: "0.2"
type: Class
title: BoundedDiagnosticOutbox
description: "Bounded, non-blocking telemetry outbox that drops records on full queue without blocking caller"
resource: crates/oxide-security/src/bounded_outbox.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/bounded_outbox/BoundedDiagnosticOutbox
language: rust
---

# BoundedDiagnosticOutbox

Bounded, non-blocking telemetry outbox that drops records on full queue without blocking caller

## Signature

```rust
pub struct BoundedDiagnosticOutbox
```

## Visibility

- `pub`

## Docstring

Bounded, non-blocking telemetry outbox that drops records on full queue without blocking caller

## Methods

- `tx`
- `dropped_count`
- `log_path`

## Source
Lines 21–25 in `crates/oxide-security/src/bounded_outbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bounded_outbox](/crates/oxide-security/src/bounded_outbox.md) |
