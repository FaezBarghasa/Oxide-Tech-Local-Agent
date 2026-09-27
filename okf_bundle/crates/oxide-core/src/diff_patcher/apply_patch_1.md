---
okf_version: "0.2"
type: Function
title: apply_patch
description: Apply unified diff hunks with fuzzy context resolution to source code
resource: crates/oxide-core/src/diff_patcher.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:07:46Z"
concept_id: crates/oxide-core/src/diff_patcher/apply_patch_1
language: rust
---

# apply_patch

Apply unified diff hunks with fuzzy context resolution to source code

## Signature

```rust
pub fn apply_patch(&self, original: &str, diff_text: &str) -> Result<PatchResult, PatchError>
```

## Visibility

- `pub`

## Docstring

Apply unified diff hunks with fuzzy context resolution to source code

## Source
Lines 132–196 in `crates/oxide-core/src/diff_patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff_patcher](/crates/oxide-core/src/diff_patcher.md) |
