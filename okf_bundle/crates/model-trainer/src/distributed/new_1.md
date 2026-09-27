---
okf_version: "0.2"
type: Function
title: new
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/new_1
language: rust
---

# new

## Signature

```rust
pub fn new(rank: usize, world_size: usize, node_id: usize, peers: Vec<String>) -> Self
```

## Visibility

- `pub`

## Source
Lines 31–38 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
