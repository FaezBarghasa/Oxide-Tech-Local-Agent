---
okf_version: "0.2"
type: Function
title: create_branch
description: Create and checkout a new task branch
resource: crates/verifier/src/git_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/git_engine/create_branch
language: rust
---

# create_branch

Create and checkout a new task branch

## Signature

```rust
impl GitEngine { pub fn create_branch(work_dir: &str, branch_name: &str) -> Result<String, String> }
```

## Visibility

- `pub`

## Docstring

Create and checkout a new task branch

## Source
Lines 61–73 in `crates/verifier/src/git_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git_engine](/crates/verifier/src/git_engine.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
