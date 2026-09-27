---
okf_version: "0.2"
type: Function
title: status
description: Get current working branch and modified / untracked files
resource: crates/verifier/src/git_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/git_engine/status_1
language: rust
---

# status

Get current working branch and modified / untracked files

## Signature

```rust
pub fn status(work_dir: &str) -> Result<GitStatusResult, String>
```

## Visibility

- `pub`

## Docstring

Get current working branch and modified / untracked files

## Source
Lines 25–58 in `crates/verifier/src/git_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git_engine](/crates/verifier/src/git_engine.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
