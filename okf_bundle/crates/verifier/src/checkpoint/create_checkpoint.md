---
okf_version: "0.2"
type: Function
title: create_checkpoint
description: "Create an in-memory & git-backed checkpoint before mutating files"
resource: crates/verifier/src/checkpoint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/checkpoint/create_checkpoint
language: rust
---

# create_checkpoint

Create an in-memory & git-backed checkpoint before mutating files

## Signature

```rust
impl CheckpointManager { pub fn create_checkpoint(
        checkpoint_id: &str,
        description: &str,
        work_dir: &str,
    ) -> Result<WorkspaceCheckpoint, String> }
```

## Visibility

- `pub`

## Docstring

Create an in-memory & git-backed checkpoint before mutating files

## Source
Lines 27–48 in `crates/verifier/src/checkpoint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [checkpoint](/crates/verifier/src/checkpoint.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
