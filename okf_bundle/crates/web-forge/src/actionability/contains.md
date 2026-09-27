---
okf_version: "0.2"
type: Function
title: contains
description: Check if point is inside bounding box.
resource: crates/web-forge/src/actionability.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:37Z"
concept_id: crates/web-forge/src/actionability/contains
language: rust
---

# contains

Check if point is inside bounding box.

## Signature

```rust
impl BoundingBox { pub fn contains(&self, px: f64, py: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if point is inside bounding box.

## Source
Lines 37–39 in `crates/web-forge/src/actionability.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actionability](/crates/web-forge/src/actionability.md) |
