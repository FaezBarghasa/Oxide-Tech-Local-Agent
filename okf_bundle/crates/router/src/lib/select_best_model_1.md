---
okf_version: "0.2"
type: Function
title: select_best_model
description: "Select the model that scores the highest based on quality, latency, cost, and success rate."
resource: crates/router/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/lib/select_best_model_1
language: rust
---

# select_best_model

Select the model that scores the highest based on quality, latency, cost, and success rate.

## Signature

```rust
pub fn select_best_model(models: &[ModelInfo]) -> Option<ModelInfo>
```

## Visibility

- `pub`

## Docstring

Select the model that scores the highest based on quality, latency, cost, and success rate.

## Source
Lines 48–72 in `crates/router/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/router/src/lib.md) |
