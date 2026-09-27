---
okf_version: "0.2"
type: Class
title: SessionSupervisor
description: Process Supervisor managing idempotent sessions and single-attempt crash recovery
resource: crates/oxide-security/src/session_supervisor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/session_supervisor/SessionSupervisor
language: rust
---

# SessionSupervisor

Process Supervisor managing idempotent sessions and single-attempt crash recovery

## Signature

```rust
pub struct SessionSupervisor
```

## Visibility

- `pub`

## Docstring

Process Supervisor managing idempotent sessions and single-attempt crash recovery

## Methods

- `receipt_dir`
- `sessions`
- `request_to_session`

## Source
Lines 60–64 in `crates/oxide-security/src/session_supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session_supervisor](/crates/oxide-security/src/session_supervisor.md) |
