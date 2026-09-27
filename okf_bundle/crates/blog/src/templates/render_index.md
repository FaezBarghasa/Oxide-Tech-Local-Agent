---
okf_version: "0.2"
type: Function
title: render_index
resource: crates/blog/src/templates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:blog"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/blog/src/templates/render_index
language: rust
---

# render_index

## Signature

```rust
pub fn render_index(
    posts: &[BlogPost],
    page: usize,
    total_pages: usize,
    config: &BlogConfig,
) -> String
```

## Visibility

- `pub`

## Source
Lines 281–371 in `crates/blog/src/templates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [templates](/crates/blog/src/templates.md) |
| calls | [base_template](/crates/blog/src/templates/base_template.md) |
