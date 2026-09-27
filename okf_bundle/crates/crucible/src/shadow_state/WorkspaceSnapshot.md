---
okf_version: "0.2"
type: Class
title: WorkspaceSnapshot
description: "Serializable, zero-copy workspace snapshot for high-speed MCTS branch simulation."
resource: crates/crucible/src/shadow_state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:55:39Z"
concept_id: crates/crucible/src/shadow_state/WorkspaceSnapshot
language: rust
---

# WorkspaceSnapshot

Serializable, zero-copy workspace snapshot for high-speed MCTS branch simulation.

## Signature

```rust
pub struct WorkspaceSnapshot
```

## Decorators

- `derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq)`

## Visibility

- `pub`

## Docstring

Serializable, zero-copy workspace snapshot for high-speed MCTS branch simulation.
[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq)]

## Methods

- `task_id`
- `step_index`
- `registers`
- `file_digests`
- `ast_complexity_score`

## Source
Lines 5–11 in `crates/crucible/src/shadow_state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_state](/crates/crucible/src/shadow_state.md) |
