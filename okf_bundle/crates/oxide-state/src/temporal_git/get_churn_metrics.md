---
okf_version: "0.2"
type: Function
title: get_churn_metrics
description: Calculate churn metrics and co-change predictions for a file
resource: crates/oxide-state/src/temporal_git.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/temporal_git/get_churn_metrics
language: rust
---

# get_churn_metrics

Calculate churn metrics and co-change predictions for a file

## Signature

```rust
impl TemporalGitMemory { pub fn get_churn_metrics(&self, file_path: &str) -> ModuleChurnMetrics }
```

## Visibility

- `pub`

## Docstring

Calculate churn metrics and co-change predictions for a file

## Source
Lines 73–102 in `crates/oxide-state/src/temporal_git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [temporal_git](/crates/oxide-state/src/temporal_git.md) |
