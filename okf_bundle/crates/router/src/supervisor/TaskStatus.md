---
okf_version: "0.2"
type: Class
title: TaskStatus
description: Status of an individual task node in the execution DAG.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/TaskStatus
language: rust
---

# TaskStatus

Status of an individual task node in the execution DAG.

## Signature

```rust
pub enum TaskStatus
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Status of an individual task node in the execution DAG.
[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

## Methods

- `inbox_id`

## Source
Lines 73–83 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
