---
okf_version: "0.2"
type: Function
title: record_tool_call
description: Record a tool invocation and success/failure status.
resource: crates/optio/src/budget.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:40:30Z"
concept_id: crates/optio/src/budget/record_tool_call
language: rust
---

# record_tool_call

Record a tool invocation and success/failure status.

## Signature

```rust
impl BudgetTracker { pub fn record_tool_call(&self, success: bool) -> Result<(), BudgetViolation> }
```

## Visibility

- `pub`

## Docstring

Record a tool invocation and success/failure status.

## Source
Lines 109–131 in `crates/optio/src/budget.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [budget](/crates/optio/src/budget.md) |
