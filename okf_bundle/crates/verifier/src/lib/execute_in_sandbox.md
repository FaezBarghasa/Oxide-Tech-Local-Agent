---
okf_version: "0.2"
type: Function
title: execute_in_sandbox
resource: crates/verifier/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/lib/execute_in_sandbox
language: rust
---

# execute_in_sandbox

## Signature

```rust
pub fn execute_in_sandbox(cmd: &[&str], work_dir: &str) -> Result<ExecutionResult>
```

## Visibility

- `pub`

## Source
Lines 19–122 in `crates/verifier/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/verifier/src/lib.md) |
| called_by | [create_checkpoint](/crates/verifier/src/checkpoint/create_checkpoint.md) |
| called_by | [rollback](/crates/verifier/src/checkpoint/rollback.md) |
| called_by | [add](/crates/verifier/src/git_engine/add.md) |
| called_by | [commit](/crates/verifier/src/git_engine/commit.md) |
| called_by | [create_branch](/crates/verifier/src/git_engine/create_branch.md) |
| called_by | [diff](/crates/verifier/src/git_engine/diff.md) |
| called_by | [status](/crates/verifier/src/git_engine/status.md) |
| called_by | [verify_in_sandbox](/crates/verifier/src/lib/verify_in_sandbox.md) |
| called_by | [copy_to_remote](/crates/verifier/src/remote_ssh/copy_to_remote.md) |
| called_by | [execute](/crates/verifier/src/remote_ssh/execute.md) |
