---
okf_version: "0.2"
type: Class
title: MemoryPager
description: Manages hierarchical virtual paging between fast memory and archival stores.
resource: crates/oxide-state/src/pager.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/pager/MemoryPager
language: rust
---

# MemoryPager

Manages hierarchical virtual paging between fast memory and archival stores.

## Signature

```rust
pub struct MemoryPager
```

## Visibility

- `pub`

## Docstring

Manages hierarchical virtual paging between fast memory and archival stores.

## Methods

- `max_hot_tokens`
- `current_hot_tokens`
- `lru_cache`

## Source
Lines 28–32 in `crates/oxide-state/src/pager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pager](/crates/oxide-state/src/pager.md) |
