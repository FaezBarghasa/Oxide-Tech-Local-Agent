---
okf_version: "0.2"
type: Function
title: add
description: Stage specific files or all changes
resource: crates/verifier/src/git_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/git_engine/add_1
language: rust
---

# add

Stage specific files or all changes

## Signature

```rust
pub fn add(work_dir: &str, files: Option<&[&str]>) -> Result<String, String>
```

## Visibility

- `pub`

## Docstring

Stage specific files or all changes

## Source
Lines 76–93 in `crates/verifier/src/git_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git_engine](/crates/verifier/src/git_engine.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
