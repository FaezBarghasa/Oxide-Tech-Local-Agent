---
okf_version: "0.2"
type: Class
title: GitCommitInfo
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
concept_id: crates/verifier/src/git_engine/GitCommitInfo
language: rust
---

# GitCommitInfo

[derive(Debug, Serialize, Deserialize, Clone)]

## Signature

```rust
pub struct GitCommitInfo
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `hash`
- `author`
- `message`
- `timestamp`

## Source
Lines 6–11 in `crates/verifier/src/git_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git_engine](/crates/verifier/src/git_engine.md) |
