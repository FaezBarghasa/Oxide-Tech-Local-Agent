---
okf_version: "0.2"
type: Function
title: resolve
description: Resolve a pending HITL item (approved or denied) and notify the parked task.
resource: crates/scheduler/src/inbox.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/inbox/resolve
language: rust
---

# resolve

Resolve a pending HITL item (approved or denied) and notify the parked task.

## Signature

```rust
impl HitlInboxManager { pub fn resolve(&self, inbox_id: Uuid, approved: bool) -> Result<bool, anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Resolve a pending HITL item (approved or denied) and notify the parked task.

## Source
Lines 92–116 in `crates/scheduler/src/inbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inbox](/crates/scheduler/src/inbox.md) |
