---
okf_version: "0.2"
type: Function
title: new_actionable
description: Helper to create a fully actionable button/element.
resource: crates/web-forge/src/actionability.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:37Z"
concept_id: crates/web-forge/src/actionability/new_actionable_1
language: rust
---

# new_actionable

Helper to create a fully actionable button/element.

## Signature

```rust
pub fn new_actionable(node_id: u64, selector: impl Into<String>, bbox: BoundingBox) -> Self
```

## Visibility

- `pub`

## Docstring

Helper to create a fully actionable button/element.

## Source
Lines 66–78 in `crates/web-forge/src/actionability.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actionability](/crates/web-forge/src/actionability.md) |
