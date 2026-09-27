---
okf_version: "0.2"
type: Function
title: record_tokens
description: Record tokens used and check budget thresholds.
resource: crates/optio/src/budget.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:40:30Z"
concept_id: crates/optio/src/budget/record_tokens
language: rust
---

# record_tokens

Record tokens used and check budget thresholds.

## Signature

```rust
impl BudgetTracker { pub fn record_tokens(&self, tokens: usize) -> Result<(), BudgetViolation> }
```

## Visibility

- `pub`

## Docstring

Record tokens used and check budget thresholds.

## Source
Lines 87–106 in `crates/optio/src/budget.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [budget](/crates/optio/src/budget.md) |
