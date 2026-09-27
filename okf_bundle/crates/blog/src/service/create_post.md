---
okf_version: "0.2"
type: Function
title: create_post
resource: crates/blog/src/service.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/service/create_post
language: rust
---

# create_post

## Signature

```rust
pub fn create_post(db: &Surreal<Any>, post: BlogPost) -> Result<BlogPost>
```

## Visibility

- `pub`

## Source
Lines 7–23 in `crates/blog/src/service.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [service](/crates/blog/src/service.md) |
| called_by | [curate_and_publish_news](/crates/scheduler/src/lib/curate_and_publish_news.md) |
