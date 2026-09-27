---
okf_version: "0.2"
type: Class
title: ModuleChurnMetrics
description: Churn score and evolutionary volatility metrics for a specific file/module
resource: crates/oxide-state/src/temporal_git.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/temporal_git/ModuleChurnMetrics
language: rust
---

# ModuleChurnMetrics

Churn score and evolutionary volatility metrics for a specific file/module

## Signature

```rust
pub struct ModuleChurnMetrics
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

Churn score and evolutionary volatility metrics for a specific file/module
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `file_path`
- `modification_count`
- `churn_risk_score`
- `top_co_changed_files`

## Source
Lines 16–21 in `crates/oxide-state/src/temporal_git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [temporal_git](/crates/oxide-state/src/temporal_git.md) |
