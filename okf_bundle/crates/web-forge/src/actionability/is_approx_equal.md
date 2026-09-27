---
okf_version: "0.2"
type: Function
title: is_approx_equal
description: Check if two bounding boxes are approximately equal within an epsilon.
resource: crates/web-forge/src/actionability.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:37Z"
concept_id: crates/web-forge/src/actionability/is_approx_equal
language: rust
---

# is_approx_equal

Check if two bounding boxes are approximately equal within an epsilon.

## Signature

```rust
impl BoundingBox { pub fn is_approx_equal(&self, other: &BoundingBox, epsilon: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if two bounding boxes are approximately equal within an epsilon.

## Source
Lines 42–47 in `crates/web-forge/src/actionability.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actionability](/crates/web-forge/src/actionability.md) |
