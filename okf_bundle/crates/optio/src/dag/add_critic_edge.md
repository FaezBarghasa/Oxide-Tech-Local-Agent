---
okf_version: "0.2"
type: Function
title: add_critic_edge
resource: crates/optio/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/optio/src/dag/add_critic_edge
language: rust
---

# add_critic_edge

## Signature

```rust
impl TaskDag { pub fn add_critic_edge(&mut self, task_id: &str, critic_task_id: &str) }
```

## Visibility

- `pub`

## Source
Lines 48–51 in `crates/optio/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/optio/src/dag.md) |
