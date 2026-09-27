---
okf_version: "0.2"
type: Function
title: list_by_tag
resource: crates/blog/src/service.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/service/list_by_tag
language: rust
---

# list_by_tag

## Signature

```rust
pub fn list_by_tag(db: &Surreal<Any>, tag: &str) -> Result<Vec<BlogPost>>
```

## Visibility

- `pub`

## Source
Lines 65–78 in `crates/blog/src/service.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [service](/crates/blog/src/service.md) |
