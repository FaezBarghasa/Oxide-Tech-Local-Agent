---
okf_version: "0.2"
type: Function
title: submit_request
description: Register a pending HITL request and return a receiver channel that awaits human decision.
resource: crates/scheduler/src/inbox.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/inbox/submit_request_1
language: rust
---

# submit_request

Register a pending HITL request and return a receiver channel that awaits human decision.

## Signature

```rust
pub fn submit_request(
        &self,
        dag_id: Uuid,
        task_id: &str,
        reason: &str,
        risk_class: &str,
        action_details: &str,
    ) -> Result<(Uuid, oneshot::Receiver<bool>), surrealdb::Error>
```

## Visibility

- `pub`

## Docstring

Register a pending HITL request and return a receiver channel that awaits human decision.

## Source
Lines 57–89 in `crates/scheduler/src/inbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inbox](/crates/scheduler/src/inbox.md) |
