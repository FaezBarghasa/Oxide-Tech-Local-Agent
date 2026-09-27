---
okf_version: "0.2"
type: Class
title: PartitionedTensor
description: Distributed Tensor Partition
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/PartitionedTensor
language: rust
---

# PartitionedTensor

Distributed Tensor Partition

## Signature

```rust
pub struct PartitionedTensor
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Distributed Tensor Partition
[derive(Debug, Clone)]

## Methods

- `name`
- `total_elements`
- `local_slice`
- `rank_owner`

## Source
Lines 47–52 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
