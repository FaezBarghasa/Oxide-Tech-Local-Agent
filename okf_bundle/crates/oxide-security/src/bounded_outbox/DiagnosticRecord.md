---
okf_version: "0.2"
type: Class
title: DiagnosticRecord
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-security/src/bounded_outbox.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/bounded_outbox/DiagnosticRecord
language: rust
---

# DiagnosticRecord

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct DiagnosticRecord
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `level`
- `target`
- `message`
- `timestamp_ms`

## Source
Lines 13–18 in `crates/oxide-security/src/bounded_outbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bounded_outbox](/crates/oxide-security/src/bounded_outbox.md) |
