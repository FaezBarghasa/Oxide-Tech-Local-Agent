---
okf_version: "0.2"
type: Function
title: verify_hunk_match_at
resource: crates/oxide-core/src/diff_patcher.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:07:46Z"
concept_id: crates/oxide-core/src/diff_patcher/verify_hunk_match_at
language: rust
---

# verify_hunk_match_at

## Signature

```rust
impl UnifiedDiffPatcher { fn verify_hunk_match_at(&self, source_lines: &[String], hunk: &DiffHunk, start_idx: usize) -> bool }
```

## Source
Lines 224–238 in `crates/oxide-core/src/diff_patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff_patcher](/crates/oxide-core/src/diff_patcher.md) |
