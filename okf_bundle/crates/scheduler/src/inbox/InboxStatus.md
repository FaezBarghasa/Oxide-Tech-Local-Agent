---
okf_version: "0.2"
type: Class
title: InboxStatus
description: State of an inbox item.
resource: crates/scheduler/src/inbox.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/inbox/InboxStatus
language: rust
---

# InboxStatus

State of an inbox item.

## Signature

```rust
pub enum InboxStatus
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, SurrealValue)`

## Visibility

- `pub`

## Docstring

State of an inbox item.
[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, SurrealValue)]

## Source
Lines 17–22 in `crates/scheduler/src/inbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inbox](/crates/scheduler/src/inbox.md) |
