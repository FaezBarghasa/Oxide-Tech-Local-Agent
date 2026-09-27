---
okf_version: "0.2"
type: Class
title: AstParser
description: AST parser that watches a directory of Rust source files and extracts a simple symbol
resource: crates/rag-pipeline/src/ast_parser.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:12Z"
concept_id: crates/rag-pipeline/src/ast_parser/AstParser
language: rust
---

# AstParser

AST parser that watches a directory of Rust source files and extracts a simple symbol

## Signature

```rust
pub struct AstParser
```

## Visibility

- `pub`

## Docstring

AST parser that watches a directory of Rust source files and extracts a simple symbol
table (function names, struct names, enum names). The resulting symbol table is stored
in an in‑memory `HashMap` keyed by the file path.

## Methods

- `root`
- `symbols`
- `watcher`

## Source
Lines 11–18 in `crates/rag-pipeline/src/ast_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast_parser](/crates/rag-pipeline/src/ast_parser.md) |
