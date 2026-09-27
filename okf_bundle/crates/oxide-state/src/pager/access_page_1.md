---
okf_version: "0.2"
type: Function
title: access_page
description: "Load or retrieve a page into the Hot tier, evicting older pages if token budget exceeded."
resource: crates/oxide-state/src/pager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/pager/access_page_1
language: rust
---

# access_page

Load or retrieve a page into the Hot tier, evicting older pages if token budget exceeded.

## Signature

```rust
pub fn access_page(&mut self, page_id: &str) -> Option<VirtualPage>
```

## Visibility

- `pub`

## Docstring

Load or retrieve a page into the Hot tier, evicting older pages if token budget exceeded.

## Source
Lines 45–51 in `crates/oxide-state/src/pager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pager](/crates/oxide-state/src/pager.md) |
