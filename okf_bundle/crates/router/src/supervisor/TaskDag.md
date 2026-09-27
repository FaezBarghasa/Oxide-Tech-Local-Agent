---
okf_version: "0.2"
type: Class
title: TaskDag
description: Directed Acyclic Graph (DAG) for multi-agent coordination.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/TaskDag
language: rust
---

# TaskDag

Directed Acyclic Graph (DAG) for multi-agent coordination.

## Signature

```rust
pub struct TaskDag
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, Default)`

## Visibility

- `pub`

## Docstring

Directed Acyclic Graph (DAG) for multi-agent coordination.
[derive(Debug, Serialize, Deserialize, Clone, Default)]

## Methods

- `dag_id`
- `nodes`
- `execution_order`

## Source
Lines 105–109 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
