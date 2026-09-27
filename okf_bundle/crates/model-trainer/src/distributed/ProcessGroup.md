---
okf_version: "0.2"
type: Class
title: ProcessGroup
description: Cluster Node / Process Group Description
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/ProcessGroup
language: rust
---

# ProcessGroup

Cluster Node / Process Group Description

## Signature

```rust
pub struct ProcessGroup
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Cluster Node / Process Group Description
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `rank`
- `world_size`
- `node_id`
- `peers`

## Source
Lines 23–28 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
