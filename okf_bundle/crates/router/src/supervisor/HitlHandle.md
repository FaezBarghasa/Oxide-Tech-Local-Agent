---
okf_version: "0.2"
type: Class
title: HitlHandle
description: A pending HITL approval that can suspend and resume a task.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/HitlHandle
language: rust
---

# HitlHandle

A pending HITL approval that can suspend and resume a task.

## Signature

```rust
pub struct HitlHandle
```

## Visibility

- `pub`

## Docstring

A pending HITL approval that can suspend and resume a task.

## Methods

- `inbox_id`
- `task_id`
- `reason`
- `resume_tx`

## Source
Lines 201–207 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
