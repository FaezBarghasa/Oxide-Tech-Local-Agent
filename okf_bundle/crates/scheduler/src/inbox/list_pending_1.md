---
okf_version: "0.2"
type: Function
title: list_pending
description: Retrieve all pending requests from the database.
resource: crates/scheduler/src/inbox.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/inbox/list_pending_1
language: rust
---

# list_pending

Retrieve all pending requests from the database.

## Signature

```rust
pub fn list_pending(&self) -> Result<Vec<InboxEntry>, surrealdb::Error>
```

## Visibility

- `pub`

## Docstring

Retrieve all pending requests from the database.

## Source
Lines 119–126 in `crates/scheduler/src/inbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inbox](/crates/scheduler/src/inbox.md) |
