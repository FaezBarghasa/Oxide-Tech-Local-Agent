---
okf_version: "0.2"
type: Class
title: TemporalGitMemory
description: "Temporal Git Engine: Maintains evolutionary history and co-change coupling"
resource: crates/oxide-state/src/temporal_git.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/temporal_git/TemporalGitMemory
language: rust
---

# TemporalGitMemory

Temporal Git Engine: Maintains evolutionary history and co-change coupling

## Signature

```rust
pub struct TemporalGitMemory
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, Default)`

## Visibility

- `pub`

## Docstring

Temporal Git Engine: Maintains evolutionary history and co-change coupling
[derive(Debug, Serialize, Deserialize, Clone, Default)]

## Methods

- `commits`
- `file_edit_counts`
- `co_change_matrix`

## Source
Lines 25–29 in `crates/oxide-state/src/temporal_git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [temporal_git](/crates/oxide-state/src/temporal_git.md) |
