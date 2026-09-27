---
okf_version: "0.2"
type: Class
title: TieredBuffer
description: Metadata and storage descriptor for offloaded or tier-managed tensors.
resource: crates/model-trainer/src/ddr5_offload.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/ddr5_offload/TieredBuffer
language: rust
---

# TieredBuffer

Metadata and storage descriptor for offloaded or tier-managed tensors.

## Signature

```rust
pub struct TieredBuffer
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Metadata and storage descriptor for offloaded or tier-managed tensors.
[derive(Debug, Clone)]

## Methods

- `name`
- `tier`
- `size_bytes`
- `host_buffer`
- `device_ptr`

## Source
Lines 19–27 in `crates/model-trainer/src/ddr5_offload.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ddr5_offload](/crates/model-trainer/src/ddr5_offload.md) |
