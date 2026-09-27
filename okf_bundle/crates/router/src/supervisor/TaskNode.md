---
okf_version: "0.2"
type: Class
title: TaskNode
description: A node in the execution DAG representing a delegated sub-agent task.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/TaskNode
language: rust
---

# TaskNode

A node in the execution DAG representing a delegated sub-agent task.

## Signature

```rust
pub struct TaskNode
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

A node in the execution DAG representing a delegated sub-agent task.
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `id`
- `title`
- `role`
- `description`
- `dependencies`
- `status`
- `result`
- `retry_count`

## Source
Lines 89–99 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
