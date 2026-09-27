---
okf_version: "0.2"
type: Class
title: AXNode
description: An Accessibility Tree node representing a semantic web element.
resource: crates/web-forge/src/distiller.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:19Z"
concept_id: crates/web-forge/src/distiller/AXNode
language: rust
---

# AXNode

An Accessibility Tree node representing a semantic web element.

## Signature

```rust
pub struct AXNode
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

An Accessibility Tree node representing a semantic web element.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `node_id`
- `role`
- `name`
- `value`
- `description`
- `disabled`
- `checked`
- `expanded`
- `ignored`
- `children`

## Source
Lines 6–22 in `crates/web-forge/src/distiller.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distiller](/crates/web-forge/src/distiller.md) |
