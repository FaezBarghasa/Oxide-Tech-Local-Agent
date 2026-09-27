---
okf_version: "0.2"
type: Class
title: ElementLayoutState
description: "Snapshot of an element's physical and computed layout state."
resource: crates/web-forge/src/actionability.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:37Z"
concept_id: crates/web-forge/src/actionability/ElementLayoutState
language: rust
---

# ElementLayoutState

Snapshot of an element's physical and computed layout state.

## Signature

```rust
pub struct ElementLayoutState
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Snapshot of an element's physical and computed layout state.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `node_id`
- `selector`
- `attached`
- `bounding_box`
- `visible`
- `enabled`
- `opacity`
- `display`
- `visibility`

## Source
Lines 52–62 in `crates/web-forge/src/actionability.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actionability](/crates/web-forge/src/actionability.md) |
