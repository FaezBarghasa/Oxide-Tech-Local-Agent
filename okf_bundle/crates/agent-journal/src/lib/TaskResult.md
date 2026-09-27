---
okf_version: "0.2"
type: Class
title: TaskResult
description: "Rich, typed result for a completed `TaskNode`."
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/TaskResult
language: rust
---

# TaskResult

Rich, typed result for a completed `TaskNode`.

## Signature

```rust
pub struct TaskResult
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, SurrealValue)`

## Visibility

- `pub`

## Docstring

Rich, typed result for a completed `TaskNode`.
Replaces the previous bare `Option<String>` in `supervisor.rs`.
[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]

## Methods

- `summary`
- `artifacts`
- `token_cost`
- `latency_ms`
- `confidence`
- `eval_score`

## Source
Lines 48–61 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
