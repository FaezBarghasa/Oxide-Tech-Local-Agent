---
okf_version: "0.2"
type: Class
title: TemporalCommit
description: Commit metadata representation in temporal memory
resource: crates/oxide-state/src/temporal_git.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/temporal_git/TemporalCommit
language: rust
---

# TemporalCommit

Commit metadata representation in temporal memory

## Signature

```rust
pub struct TemporalCommit
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

Commit metadata representation in temporal memory
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `hash`
- `author`
- `message`
- `timestamp`
- `files_changed`

## Source
Lines 6–12 in `crates/oxide-state/src/temporal_git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [temporal_git](/crates/oxide-state/src/temporal_git.md) |
