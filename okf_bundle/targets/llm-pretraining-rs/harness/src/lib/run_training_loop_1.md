---
okf_version: "0.2"
type: Function
title: run_training_loop
description: Execute training loop on sealed validation split
resource: targets/llm-pretraining-rs/harness/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/harness/src/lib/run_training_loop_1
language: rust
---

# run_training_loop

Execute training loop on sealed validation split

## Signature

```rust
pub fn run_training_loop(&self, loader: &SurfaceLoader) -> Result<f64, LoaderError>
```

## Visibility

- `pub`

## Docstring

Execute training loop on sealed validation split

## Source
Lines 123–154 in `targets/llm-pretraining-rs/harness/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/harness/src/lib.md) |
