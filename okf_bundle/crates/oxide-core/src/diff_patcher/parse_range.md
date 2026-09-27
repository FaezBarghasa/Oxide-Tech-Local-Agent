---
okf_version: "0.2"
type: Function
title: parse_range
resource: crates/oxide-core/src/diff_patcher.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:07:46Z"
concept_id: crates/oxide-core/src/diff_patcher/parse_range
language: rust
---

# parse_range

## Signature

```rust
impl UnifiedDiffPatcher { fn parse_range(range_str: &str) -> Result<(usize, usize), PatchError> }
```

## Source
Lines 115–129 in `crates/oxide-core/src/diff_patcher.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff_patcher](/crates/oxide-core/src/diff_patcher.md) |
