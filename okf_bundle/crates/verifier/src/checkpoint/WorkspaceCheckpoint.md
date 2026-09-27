---
okf_version: "0.2"
type: Class
title: WorkspaceCheckpoint
description: Checkpoint recording the state of the workspace prior to an action
resource: crates/verifier/src/checkpoint.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/checkpoint/WorkspaceCheckpoint
language: rust
---

# WorkspaceCheckpoint

Checkpoint recording the state of the workspace prior to an action

## Signature

```rust
pub struct WorkspaceCheckpoint
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

Checkpoint recording the state of the workspace prior to an action
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `checkpoint_id`
- `description`
- `timestamp`
- `files`
- `git_stash_ref`

## Source
Lines 15–21 in `crates/verifier/src/checkpoint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [checkpoint](/crates/verifier/src/checkpoint.md) |
