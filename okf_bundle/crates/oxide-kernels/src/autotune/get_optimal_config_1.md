---
okf_version: "0.2"
type: Function
title: get_optimal_config
description: Select optimal kernel tile size and pipeline configuration
resource: crates/oxide-kernels/src/autotune.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:32Z"
concept_id: crates/oxide-kernels/src/autotune/get_optimal_config_1
language: rust
---

# get_optimal_config

Select optimal kernel tile size and pipeline configuration

## Signature

```rust
pub fn get_optimal_config(&self, m: usize, n: usize, k: usize, is_fp8: bool) -> KernelConfig
```

## Visibility

- `pub`

## Docstring

Select optimal kernel tile size and pipeline configuration

## Source
Lines 118–140 in `crates/oxide-kernels/src/autotune.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autotune](/crates/oxide-kernels/src/autotune.md) |
