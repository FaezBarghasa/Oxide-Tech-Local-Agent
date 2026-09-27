---
okf_version: "0.2"
type: Class
title: InboxEntry
description: An inbox entry persisted to SurrealDB representing an action waiting for human review.
resource: crates/scheduler/src/inbox.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/inbox/InboxEntry
language: rust
---

# InboxEntry

An inbox entry persisted to SurrealDB representing an action waiting for human review.

## Signature

```rust
pub struct InboxEntry
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, SurrealValue)`

## Visibility

- `pub`

## Docstring

An inbox entry persisted to SurrealDB representing an action waiting for human review.
[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]

## Methods

- `id`
- `inbox_id`
- `dag_id`
- `task_id`
- `reason`
- `risk_class`
- `action_details`
- `status`
- `created_at`
- `resolved_at`

## Source
Lines 26–38 in `crates/scheduler/src/inbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inbox](/crates/scheduler/src/inbox.md) |
