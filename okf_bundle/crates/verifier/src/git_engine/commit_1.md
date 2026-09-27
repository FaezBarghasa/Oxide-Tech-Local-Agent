---
okf_version: "0.2"
type: Function
title: commit
description: Author a semantic commit
resource: crates/verifier/src/git_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/git_engine/commit_1
language: rust
---

# commit

Author a semantic commit

## Signature

```rust
pub fn commit(work_dir: &str, message: &str) -> Result<String, String>
```

## Visibility

- `pub`

## Docstring

Author a semantic commit

## Source
Lines 96–106 in `crates/verifier/src/git_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git_engine](/crates/verifier/src/git_engine.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
