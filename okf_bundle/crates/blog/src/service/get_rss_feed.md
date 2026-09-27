---
okf_version: "0.2"
type: Function
title: get_rss_feed
resource: crates/blog/src/service.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/service/get_rss_feed
language: rust
---

# get_rss_feed

## Signature

```rust
pub fn get_rss_feed(
    db: &Surreal<Any>,
    site_name: &str,
    description: &str,
    base_url: &str,
) -> Result<String>
```

## Visibility

- `pub`

## Source
Lines 80–143 in `crates/blog/src/service.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [service](/crates/blog/src/service.md) |
| calls | [list_posts](/crates/blog/src/service/list_posts.md) |
