---
okf_version: "0.2"
type: Function
title: find_hunk_offset
description: Locate hunk position considering small upstream offsets
resource: crates/oxide-core/src/diff_patcher.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:07:46Z"
concept_id: crates/oxide-core/src/diff_patcher/find_hunk_offset
language: rust
---

# find_hunk_offset

Locate hunk position considering small upstream offsets

## Signature

```rust
impl UnifiedDiffPatcher { fn find_hunk_offset(&self, source_lines: &[String], hunk: &DiffHunk) -> Result<(usize, bool), PatchError> }
```

## Docstring

Locate hunk position considering small upstream offsets

## Source
Lines 199–222 in `crates/oxide-core/src/diff_patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff_patcher](/crates/oxide-core/src/diff_patcher.md) |
