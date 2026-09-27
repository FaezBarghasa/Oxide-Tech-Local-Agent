---
okf_version: "0.2"
type: Function
title: diff
description: Generate unified diff for workspace changes
resource: crates/verifier/src/git_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/git_engine/diff_1
language: rust
---

# diff

Generate unified diff for workspace changes

## Signature

```rust
pub fn diff(work_dir: &str) -> Result<String, String>
```

## Visibility

- `pub`

## Docstring

Generate unified diff for workspace changes

## Source
Lines 109–115 in `crates/verifier/src/git_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git_engine](/crates/verifier/src/git_engine.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
