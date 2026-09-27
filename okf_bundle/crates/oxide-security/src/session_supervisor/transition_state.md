---
okf_version: "0.2"
type: Function
title: transition_state
description: Transition session state and enforce single crash recovery attempt
resource: crates/oxide-security/src/session_supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/session_supervisor/transition_state
language: rust
---

# transition_state

Transition session state and enforce single crash recovery attempt

## Signature

```rust
impl SessionSupervisor { pub fn transition_state(
        &self,
        session_id: &str,
        new_state: SessionState,
    ) -> Result<SessionReceipt, SessionSupervisorError> }
```

## Visibility

- `pub`

## Docstring

Transition session state and enforce single crash recovery attempt

## Source
Lines 103–140 in `crates/oxide-security/src/session_supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session_supervisor](/crates/oxide-security/src/session_supervisor.md) |
