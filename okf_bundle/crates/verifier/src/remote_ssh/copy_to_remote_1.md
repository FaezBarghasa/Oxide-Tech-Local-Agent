---
okf_version: "0.2"
type: Function
title: copy_to_remote
description: Copy a file to remote target via SCP
resource: crates/verifier/src/remote_ssh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/remote_ssh/copy_to_remote_1
language: rust
---

# copy_to_remote

Copy a file to remote target via SCP

## Signature

```rust
pub fn copy_to_remote(
        config: &SshConfig,
        local_path: &str,
        remote_path: &str,
        work_dir: &str,
    ) -> Result<ExecutionResult, String>
```

## Visibility

- `pub`

## Docstring

Copy a file to remote target via SCP

## Source
Lines 53–85 in `crates/verifier/src/remote_ssh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_ssh](/crates/verifier/src/remote_ssh.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
