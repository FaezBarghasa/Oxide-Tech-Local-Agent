---
okf_version: "0.2"
type: Function
title: admit_or_resume_session
description: Obtain or create an idempotent session receipt
resource: crates/oxide-security/src/session_supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/session_supervisor/admit_or_resume_session_1
language: rust
---

# admit_or_resume_session

Obtain or create an idempotent session receipt

## Signature

```rust
pub fn admit_or_resume_session(
        &self,
        request_id: &str,
    ) -> Result<SessionReceipt, SessionSupervisorError>
```

## Visibility

- `pub`

## Docstring

Obtain or create an idempotent session receipt

## Source
Lines 78–100 in `crates/oxide-security/src/session_supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session_supervisor](/crates/oxide-security/src/session_supervisor.md) |
