---
okf_version: "0.2"
type: Class
title: GitStatusResult
description: "[derive(Debug, Serialize, Deserialize, Clone)]"
resource: crates/verifier/src/git_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/git_engine/GitStatusResult
language: rust
---

# GitStatusResult

[derive(Debug, Serialize, Deserialize, Clone)]

## Signature

```rust
pub struct GitStatusResult
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `branch`
- `clean`
- `modified_files`
- `untracked_files`

## Source
Lines 14–19 in `crates/verifier/src/git_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git_engine](/crates/verifier/src/git_engine.md) |
