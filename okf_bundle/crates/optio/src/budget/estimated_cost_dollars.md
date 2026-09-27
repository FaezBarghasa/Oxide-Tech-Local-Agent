---
okf_version: "0.2"
type: Function
title: estimated_cost_dollars
description: Calculate estimated dollar cost based on model blend pricing ($0.20 per 1M local tokens amortized)
resource: crates/optio/src/budget.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:40:30Z"
concept_id: crates/optio/src/budget/estimated_cost_dollars
language: rust
---

# estimated_cost_dollars

Calculate estimated dollar cost based on model blend pricing ($0.20 per 1M local tokens amortized)

## Signature

```rust
impl BudgetTracker { pub fn estimated_cost_dollars(&self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Calculate estimated dollar cost based on model blend pricing ($0.20 per 1M local tokens amortized)

## Source
Lines 146–149 in `crates/optio/src/budget.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [budget](/crates/optio/src/budget.md) |
