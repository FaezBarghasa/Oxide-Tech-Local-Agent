---
okf_version: "0.2"
type: Function
title: save_ast_cache
resource: crates/tree-sitter-service/src/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/tree-sitter-service/src/cache/save_ast_cache
language: rust
---

# save_ast_cache

## Signature

```rust
pub fn save_ast_cache(path: &std::path::Path, ast: &CachedAST) -> Result<(), anyhow::Error>
```

## Visibility

- `pub`

## Source
Lines 22–28 in `crates/tree-sitter-service/src/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/tree-sitter-service/src/cache.md) |
