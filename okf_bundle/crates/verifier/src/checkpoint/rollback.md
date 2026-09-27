---
okf_version: "0.2"
type: Function
title: rollback
description: Revert working tree instantaneously to the recorded checkpoint
resource: crates/verifier/src/checkpoint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/checkpoint/rollback
language: rust
---

# rollback

Revert working tree instantaneously to the recorded checkpoint

## Signature

```rust
impl CheckpointManager { pub fn rollback(
        checkpoint: &WorkspaceCheckpoint,
        work_dir: &str,
    ) -> Result<String, String> }
```

## Visibility

- `pub`

## Docstring

Revert working tree instantaneously to the recorded checkpoint

## Source
Lines 51–77 in `crates/verifier/src/checkpoint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [checkpoint](/crates/verifier/src/checkpoint.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
