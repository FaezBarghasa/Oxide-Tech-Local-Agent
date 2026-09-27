---
okf_version: "0.2"
type: Class
title: ReplayedDagState
description: Reconstructed DAG state after journal replay.
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/ReplayedDagState
language: rust
---

# ReplayedDagState

Reconstructed DAG state after journal replay.

## Signature

```rust
pub struct ReplayedDagState
```

## Decorators

- `derive(Debug, Default)`

## Visibility

- `pub`

## Docstring

Reconstructed DAG state after journal replay.
[derive(Debug, Default)]

## Methods

- `dag_id`
- `goal`
- `mode`
- `task_states`
- `task_results`
- `pending_hitl`
- `terminated`

## Source
Lines 245–257 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
