---
okf_version: "0.2"
type: Class
title: VirtualPage
description: A discrete unit of virtualized conversational or task context.
resource: crates/oxide-state/src/pager.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/pager/VirtualPage
language: rust
---

# VirtualPage

A discrete unit of virtualized conversational or task context.

## Signature

```rust
pub struct VirtualPage
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A discrete unit of virtualized conversational or task context.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `page_id`
- `session_id`
- `tier`
- `token_count`
- `content`
- `last_accessed`

## Source
Lines 18–25 in `crates/oxide-state/src/pager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pager](/crates/oxide-state/src/pager.md) |
