---
okf_version: "0.2"
type: Function
title: boolean_union
description: Performs CSG Boolean Union (A ∪ B) in-place or returning a new volume.
resource: crates/visual-forge/src/sdf_feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:03:08Z"
concept_id: crates/visual-forge/src/sdf_feedback/boolean_union_1
language: rust
---

# boolean_union

Performs CSG Boolean Union (A ∪ B) in-place or returning a new volume.

## Signature

```rust
pub fn boolean_union(&self, other: &SDFVolume) -> Self
```

## Visibility

- `pub`

## Docstring

Performs CSG Boolean Union (A ∪ B) in-place or returning a new volume.

## Source
Lines 289–299 in `crates/visual-forge/src/sdf_feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sdf_feedback](/crates/visual-forge/src/sdf_feedback.md) |
