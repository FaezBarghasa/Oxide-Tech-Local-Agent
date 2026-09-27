---
okf_version: "0.2"
type: Class
title: JournalEvent
description: "All possible journal events — the complete vocabulary of the agent's execution."
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/JournalEvent
language: rust
---

# JournalEvent

All possible journal events — the complete vocabulary of the agent's execution.

## Signature

```rust
pub enum JournalEvent
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, SurrealValue)`
- `serde(tag = "event_type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

All possible journal events — the complete vocabulary of the agent's execution.
[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
[serde(tag = "event_type", rename_all = "snake_case")]

## Methods

- `dag_id`
- `goal`
- `mode`
- `dag_id`
- `task_id`
- `role`
- `dag_id`
- `task_id`
- `result`
- `dag_id`
- `task_id`
- `error`
- `retry_count`
- `dag_id`
- `task_id`
- `reason`
- `dag_id`
- `git_sha`
- `dag_id`
- `task_id`
- `error_signature`
- `count`
- `dag_id`
- `task_id`
- `reason`
- `risk_class`
- `inbox_id`
- `dag_id`
- `task_id`
- `inbox_id`
- `decision`
- `dag_id`
- `outcome`
- `total_token_cost`
- `total_latency_ms`

## Source
Lines 77–140 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
