---
okf_version: "0.2"
type: Class
title: BudgetConfig
description: Budget configuration and trackers for token and tool execution governance.
resource: crates/optio/src/budget.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:40:30Z"
concept_id: crates/optio/src/budget/BudgetConfig
language: rust
---

# BudgetConfig

Budget configuration and trackers for token and tool execution governance.

## Signature

```rust
pub struct BudgetConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Budget configuration and trackers for token and tool execution governance.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `max_tokens_per_task`
- `max_tokens_per_session`
- `max_tool_calls_per_task`
- `max_consecutive_tool_failures`

## Source
Lines 7–16 in `crates/optio/src/budget.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [budget](/crates/optio/src/budget.md) |
