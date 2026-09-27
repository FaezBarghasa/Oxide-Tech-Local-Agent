---
okf_version: "0.2"
type: Function
title: dropped_records_count
resource: crates/oxide-security/src/bounded_outbox.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/bounded_outbox/dropped_records_count
language: rust
---

# dropped_records_count

## Signature

```rust
impl BoundedDiagnosticOutbox { pub fn dropped_records_count(&self) -> usize }
```

## Visibility

- `pub`

## Source
Lines 83–85 in `crates/oxide-security/src/bounded_outbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bounded_outbox](/crates/oxide-security/src/bounded_outbox.md) |
