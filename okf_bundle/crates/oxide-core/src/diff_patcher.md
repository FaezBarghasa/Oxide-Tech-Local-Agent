---
okf_version: "0.2"
type: Module
title: diff_patcher
description: "# Native Unified Diff Patching Engine"
resource: crates/oxide-core/src/diff_patcher.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:07:46Z"
concept_id: crates/oxide-core/src/diff_patcher
language: rust
---

# diff_patcher

# Native Unified Diff Patching Engine

## Docstring

# Native Unified Diff Patching Engine

Provides surgical, AST-aware hunk resolution, fuzzy offset compensation,
dry-run preflight verification, and minimal-byte file mutation without full-file rewrites.

## Relationships

| Type | Target |
|------|--------|
| related | [PatchError](/crates/oxide-core/src/diff_patcher/PatchError.md) |
| related | [DiffLine](/crates/oxide-core/src/diff_patcher/DiffLine.md) |
| related | [DiffHunk](/crates/oxide-core/src/diff_patcher/DiffHunk.md) |
| related | [PatchResult](/crates/oxide-core/src/diff_patcher/PatchResult.md) |
| related | [UnifiedDiffPatcher](/crates/oxide-core/src/diff_patcher/UnifiedDiffPatcher.md) |
| related | [default](/crates/oxide-core/src/diff_patcher/default.md) |
| related | [default](/crates/oxide-core/src/diff_patcher/default.md) |
| related | [new](/crates/oxide-core/src/diff_patcher/new.md) |
| related | [parse_unidiff](/crates/oxide-core/src/diff_patcher/parse_unidiff.md) |
| related | [parse_range](/crates/oxide-core/src/diff_patcher/parse_range.md) |
| related | [apply_patch](/crates/oxide-core/src/diff_patcher/apply_patch.md) |
| related | [find_hunk_offset](/crates/oxide-core/src/diff_patcher/find_hunk_offset.md) |
| related | [verify_hunk_match_at](/crates/oxide-core/src/diff_patcher/verify_hunk_match_at.md) |
| related | [new](/crates/oxide-core/src/diff_patcher/new.md) |
| related | [parse_unidiff](/crates/oxide-core/src/diff_patcher/parse_unidiff.md) |
| related | [parse_range](/crates/oxide-core/src/diff_patcher/parse_range.md) |
| related | [apply_patch](/crates/oxide-core/src/diff_patcher/apply_patch.md) |
| related | [find_hunk_offset](/crates/oxide-core/src/diff_patcher/find_hunk_offset.md) |
| related | [verify_hunk_match_at](/crates/oxide-core/src/diff_patcher/verify_hunk_match_at.md) |
| related | [fmt](/crates/oxide-core/src/diff_patcher/fmt.md) |
| related | [fmt](/crates/oxide-core/src/diff_patcher/fmt.md) |
| related | [test_unified_diff_exact_patch](/crates/oxide-core/src/diff_patcher/test_unified_diff_exact_patch.md) |
| related | [test_unified_diff_fuzzy_offset_resolution](/crates/oxide-core/src/diff_patcher/test_unified_diff_fuzzy_offset_resolution.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
