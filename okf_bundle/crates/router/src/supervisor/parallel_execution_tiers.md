---
okf_version: "0.2"
type: Function
title: parallel_execution_tiers
description: Group tasks that have no mutual dependencies — these can run in parallel.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/parallel_execution_tiers
language: rust
---

# parallel_execution_tiers

Group tasks that have no mutual dependencies — these can run in parallel.

## Signature

```rust
impl TaskDag { pub fn parallel_execution_tiers(&self) -> Vec<Vec<String>> }
```

## Visibility

- `pub`

## Docstring

Group tasks that have no mutual dependencies — these can run in parallel.

Returns a `Vec<Vec<String>>` where each inner vec is a parallel execution tier.
Tiers are ordered so that a later tier only starts after all earlier tiers complete.

## Source
Lines 166–195 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
