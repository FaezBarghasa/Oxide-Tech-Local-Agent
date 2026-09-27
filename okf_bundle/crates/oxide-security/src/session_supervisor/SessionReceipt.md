---
okf_version: "0.2"
type: Class
title: SessionReceipt
description: Fsynced Receipt persisted prior to acknowledging turns
resource: crates/oxide-security/src/session_supervisor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/session_supervisor/SessionReceipt
language: rust
---

# SessionReceipt

Fsynced Receipt persisted prior to acknowledging turns

## Signature

```rust
pub struct SessionReceipt
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Fsynced Receipt persisted prior to acknowledging turns
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `session_id`
- `request_id`
- `state`
- `recovery_attempts`
- `created_at_ms`
- `updated_at_ms`

## Source
Lines 36–43 in `crates/oxide-security/src/session_supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session_supervisor](/crates/oxide-security/src/session_supervisor.md) |
