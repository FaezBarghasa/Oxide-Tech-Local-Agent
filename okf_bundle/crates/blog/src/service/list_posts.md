---
okf_version: "0.2"
type: Function
title: list_posts
resource: crates/blog/src/service.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/service/list_posts
language: rust
---

# list_posts

## Signature

```rust
pub fn list_posts(db: &Surreal<Any>, page: usize, per_page: usize) -> Result<Vec<BlogPost>>
```

## Visibility

- `pub`

## Source
Lines 46–63 in `crates/blog/src/service.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [service](/crates/blog/src/service.md) |
| called_by | [get_rss_feed](/crates/blog/src/service/get_rss_feed.md) |
