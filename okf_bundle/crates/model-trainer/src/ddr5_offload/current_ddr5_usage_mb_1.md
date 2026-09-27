---
okf_version: "0.2"
type: Function
title: current_ddr5_usage_mb
description: Calculate total current DDR5 host memory allocation in Megabytes.
resource: crates/model-trainer/src/ddr5_offload.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/ddr5_offload/current_ddr5_usage_mb_1
language: rust
---

# current_ddr5_usage_mb

Calculate total current DDR5 host memory allocation in Megabytes.

## Signature

```rust
pub fn current_ddr5_usage_mb(&self) -> u64
```

## Visibility

- `pub`

## Docstring

Calculate total current DDR5 host memory allocation in Megabytes.

## Source
Lines 56–64 in `crates/model-trainer/src/ddr5_offload.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ddr5_offload](/crates/model-trainer/src/ddr5_offload.md) |
