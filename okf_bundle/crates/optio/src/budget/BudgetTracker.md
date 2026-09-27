---
okf_version: "0.2"
type: Class
title: BudgetTracker
description: Dynamic tracker for token and tool call consumption.
resource: crates/optio/src/budget.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:40:30Z"
concept_id: crates/optio/src/budget/BudgetTracker
language: rust
---

# BudgetTracker

Dynamic tracker for token and tool call consumption.

## Signature

```rust
pub struct BudgetTracker
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Dynamic tracker for token and tool call consumption.
[derive(Debug, Clone)]

## Methods

- `config`
- `session_tokens_used`
- `task_tokens_used`
- `task_tool_calls`
- `consecutive_tool_failures`

## Source
Lines 31–37 in `crates/optio/src/budget.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [budget](/crates/optio/src/budget.md) |
