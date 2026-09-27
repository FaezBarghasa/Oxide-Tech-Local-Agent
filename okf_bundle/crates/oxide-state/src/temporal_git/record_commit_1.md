---
okf_version: "0.2"
type: Function
title: record_commit
description: Record a historical or newly authored commit
resource: crates/oxide-state/src/temporal_git.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/temporal_git/record_commit_1
language: rust
---

# record_commit

Record a historical or newly authored commit

## Signature

```rust
pub fn record_commit(&mut self, commit: TemporalCommit)
```

## Visibility

- `pub`

## Docstring

Record a historical or newly authored commit

## Source
Lines 41–70 in `crates/oxide-state/src/temporal_git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [temporal_git](/crates/oxide-state/src/temporal_git.md) |
