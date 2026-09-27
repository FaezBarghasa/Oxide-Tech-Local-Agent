---
okf_version: "0.2"
type: Function
title: reset_task
description: Reset per-task counters when starting a new task node.
resource: crates/optio/src/budget.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:40:30Z"
concept_id: crates/optio/src/budget/reset_task
language: rust
---

# reset_task

Reset per-task counters when starting a new task node.

## Signature

```rust
impl BudgetTracker { pub fn reset_task(&self) }
```

## Visibility

- `pub`

## Docstring

Reset per-task counters when starting a new task node.

## Source
Lines 80–84 in `crates/optio/src/budget.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [budget](/crates/optio/src/budget.md) |
