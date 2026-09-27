---
okf_version: "0.2"
type: Function
title: plan_goal
description: Decomposes a user goal into a verified multi-agent task DAG and journals it.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/plan_goal
language: rust
---

# plan_goal

Decomposes a user goal into a verified multi-agent task DAG and journals it.

## Signature

```rust
impl SupervisorAgent { pub fn plan_goal(&self, goal: &str) -> TaskDag }
```

## Visibility

- `pub`

## Docstring

Decomposes a user goal into a verified multi-agent task DAG and journals it.

## Source
Lines 249–353 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
