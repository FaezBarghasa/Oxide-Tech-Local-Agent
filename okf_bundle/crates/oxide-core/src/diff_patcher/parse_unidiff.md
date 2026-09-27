---
okf_version: "0.2"
type: Function
title: parse_unidiff
description: Parse a unified diff string into structured DiffHunks
resource: crates/oxide-core/src/diff_patcher.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:07:46Z"
concept_id: crates/oxide-core/src/diff_patcher/parse_unidiff
language: rust
---

# parse_unidiff

Parse a unified diff string into structured DiffHunks

## Signature

```rust
impl UnifiedDiffPatcher { pub fn parse_unidiff(diff_text: &str) -> Result<Vec<DiffHunk>, PatchError> }
```

## Visibility

- `pub`

## Docstring

Parse a unified diff string into structured DiffHunks

## Source
Lines 64–113 in `crates/oxide-core/src/diff_patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff_patcher](/crates/oxide-core/src/diff_patcher.md) |
