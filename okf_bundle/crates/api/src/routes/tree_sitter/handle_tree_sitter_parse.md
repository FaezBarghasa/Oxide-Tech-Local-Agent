---
okf_version: "0.2"
type: Function
title: handle_tree_sitter_parse
description: "[post(\"/api/tree-sitter/parse\")]"
resource: crates/api/src/routes/tree_sitter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/tree_sitter/handle_tree_sitter_parse
language: rust
---

# handle_tree_sitter_parse

[post("/api/tree-sitter/parse")]

## Signature

```rust
pub fn handle_tree_sitter_parse(req: web::Json<ParseRequest>) -> impl Responder
```

## Decorators

- `post("/api/tree-sitter/parse")`

## Visibility

- `pub`

## Docstring

[post("/api/tree-sitter/parse")]

## Source
Lines 81–89 in `crates/api/src/routes/tree_sitter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_sitter](/crates/api/src/routes/tree_sitter.md) |
| calls | [run_tree_sitter_parse](/crates/api/src/routes/tree_sitter/run_tree_sitter_parse.md) |
