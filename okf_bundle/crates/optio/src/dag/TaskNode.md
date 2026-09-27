---
okf_version: "0.2"
type: Class
title: TaskNode
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/optio/src/dag.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/optio/src/dag/TaskNode
language: rust
---

# TaskNode

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct TaskNode
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `description`
- `dependencies`
- `completed`
- `reflections`

## Source
Lines 13–19 in `crates/optio/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/optio/src/dag.md) |
