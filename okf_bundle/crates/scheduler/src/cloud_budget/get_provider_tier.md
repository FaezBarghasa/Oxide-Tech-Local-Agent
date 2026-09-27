---
okf_version: "0.2"
type: Function
title: get_provider_tier
resource: crates/scheduler/src/cloud_budget.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/cloud_budget/get_provider_tier
language: rust
---

# get_provider_tier

## Signature

```rust
impl CloudBudgetManager { pub fn get_provider_tier(&self, is_critical: bool) -> &'static str }
```

## Visibility

- `pub`

## Source
Lines 36–42 in `crates/scheduler/src/cloud_budget.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cloud_budget](/crates/scheduler/src/cloud_budget.md) |
