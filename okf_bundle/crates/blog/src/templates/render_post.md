---
okf_version: "0.2"
type: Function
title: render_post
resource: crates/blog/src/templates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/templates/render_post
language: rust
---

# render_post

## Signature

```rust
pub fn render_post(post: &BlogPost, config: &BlogConfig) -> String
```

## Visibility

- `pub`

## Source
Lines 373–440 in `crates/blog/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/blog/src/templates.md) |
| calls | [base_template](/crates/blog/src/templates/base_template.md) |
| called_by | [test_blog_html_rendering](/crates/blog/src/lib/test_blog_html_rendering.md) |
