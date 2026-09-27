---
okf_version: "0.2"
type: Function
title: count_posts
resource: crates/blog/src/service.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/service/count_posts
language: rust
---

# count_posts

## Signature

```rust
pub fn count_posts(db: &Surreal<Any>) -> Result<usize>
```

## Visibility

- `pub`

## Source
Lines 153–169 in `crates/blog/src/service.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [service](/crates/blog/src/service.md) |
