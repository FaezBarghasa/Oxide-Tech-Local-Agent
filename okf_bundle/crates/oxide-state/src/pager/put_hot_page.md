---
okf_version: "0.2"
type: Function
title: put_hot_page
description: Insert or promote a page to the Hot tier.
resource: crates/oxide-state/src/pager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/pager/put_hot_page
language: rust
---

# put_hot_page

Insert or promote a page to the Hot tier.

## Signature

```rust
impl MemoryPager { pub fn put_hot_page(&mut self, mut page: VirtualPage) -> Vec<VirtualPage> }
```

## Visibility

- `pub`

## Docstring

Insert or promote a page to the Hot tier.

## Source
Lines 54–76 in `crates/oxide-state/src/pager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pager](/crates/oxide-state/src/pager.md) |
