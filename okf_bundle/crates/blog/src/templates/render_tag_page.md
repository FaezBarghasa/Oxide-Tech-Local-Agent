---
okf_version: "0.2"
type: Function
title: render_tag_page
resource: crates/blog/src/templates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/templates/render_tag_page
language: rust
---

# render_tag_page

## Signature

```rust
pub fn render_tag_page(tag: &str, posts: &[BlogPost], config: &BlogConfig) -> String
```

## Visibility

- `pub`

## Source
Lines 442–509 in `crates/blog/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/blog/src/templates.md) |
| calls | [base_template](/crates/blog/src/templates/base_template.md) |
