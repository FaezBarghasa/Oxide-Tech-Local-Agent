---
okf_version: "0.2"
type: Function
title: get_post
resource: crates/blog/src/service.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/service/get_post
language: rust
---

# get_post

## Signature

```rust
pub fn get_post(db: &Surreal<Any>, id: &str) -> Result<Option<BlogPost>>
```

## Visibility

- `pub`

## Source
Lines 25–44 in `crates/blog/src/service.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [service](/crates/blog/src/service.md) |
