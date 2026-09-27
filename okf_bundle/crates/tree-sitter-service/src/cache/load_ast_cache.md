---
okf_version: "0.2"
type: Function
title: load_ast_cache
resource: crates/tree-sitter-service/src/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/tree-sitter-service/src/cache/load_ast_cache
language: rust
---

# load_ast_cache

## Signature

```rust
pub fn load_ast_cache(path: &std::path::Path) -> Result<CachedAST, anyhow::Error>
```

## Visibility

- `pub`

## Source
Lines 30–35 in `crates/tree-sitter-service/src/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/tree-sitter-service/src/cache.md) |
