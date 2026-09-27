---
okf_version: "0.2"
type: Module
title: inbox
description: "# Human-in-the-Loop (HITL) Inbox Protocol"
resource: crates/scheduler/src/inbox.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/inbox
language: rust
---

# inbox

# Human-in-the-Loop (HITL) Inbox Protocol

## Docstring

# Human-in-the-Loop (HITL) Inbox Protocol

Stores approval requests for consequential actions (WriteLocal, Exec, External)
in SurrealDB and provides asynchronous resumption via Tokio oneshot channels.

## Relationships

| Type | Target |
|------|--------|
| related | [InboxStatus](/crates/scheduler/src/inbox/InboxStatus.md) |
| related | [InboxEntry](/crates/scheduler/src/inbox/InboxEntry.md) |
| related | [HitlInboxManager](/crates/scheduler/src/inbox/HitlInboxManager.md) |
| related | [new](/crates/scheduler/src/inbox/new.md) |
| related | [submit_request](/crates/scheduler/src/inbox/submit_request.md) |
| related | [resolve](/crates/scheduler/src/inbox/resolve.md) |
| related | [list_pending](/crates/scheduler/src/inbox/list_pending.md) |
| related | [new](/crates/scheduler/src/inbox/new.md) |
| related | [submit_request](/crates/scheduler/src/inbox/submit_request.md) |
| related | [resolve](/crates/scheduler/src/inbox/resolve.md) |
| related | [list_pending](/crates/scheduler/src/inbox/list_pending.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [surrealdb](/_dependencies/cargo/surrealdb.md) |
