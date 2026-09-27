---
okf_version: "0.2"
type: Function
title: ingest_workspace_ast
description: Ingest parsed workspace AST symbols into the vector database.
resource: crates/rag-pipeline/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:44:46Z"
concept_id: crates/rag-pipeline/src/lib/ingest_workspace_ast_1
language: rust
---

# ingest_workspace_ast

Ingest parsed workspace AST symbols into the vector database.

## Signature

```rust
pub fn ingest_workspace_ast(
        &self,
        symbols: &[ParsedSymbol],
    ) -> Result<(), anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Ingest parsed workspace AST symbols into the vector database.

## Source
Lines 216–293 in `crates/rag-pipeline/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/rag-pipeline/src/lib.md) |
| calls | [format_symbol](/crates/rag-pipeline/src/lib/format_symbol.md) |
