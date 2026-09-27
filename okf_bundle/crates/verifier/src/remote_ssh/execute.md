---
okf_version: "0.2"
type: Function
title: execute
description: Execute a command on a remote host via SSH
resource: crates/verifier/src/remote_ssh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/remote_ssh/execute
language: rust
---

# execute

Execute a command on a remote host via SSH

## Signature

```rust
impl RemoteSshManager { pub fn execute(
        config: &SshConfig,
        command: &str,
        work_dir: &str,
    ) -> Result<ExecutionResult, String> }
```

## Visibility

- `pub`

## Docstring

Execute a command on a remote host via SSH

## Source
Lines 17–50 in `crates/verifier/src/remote_ssh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_ssh](/crates/verifier/src/remote_ssh.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
