---
okf_version: "0.2"
type: Function
title: is_empty
description: Returns true if width or height is zero or negative.
resource: crates/web-forge/src/actionability.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:37Z"
concept_id: crates/web-forge/src/actionability/is_empty
language: rust
---

# is_empty

Returns true if width or height is zero or negative.

## Signature

```rust
impl BoundingBox { pub fn is_empty(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Returns true if width or height is zero or negative.

## Source
Lines 32–34 in `crates/web-forge/src/actionability.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actionability](/crates/web-forge/src/actionability.md) |
