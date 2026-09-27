---
okf_version: "0.2"
type: Function
title: ingest_workspace_ast
description: "Ingest parsed workspace AST symbols into the \"code\" collection."
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/ingest_workspace_ast_1
language: rust
---

# ingest_workspace_ast

Ingest parsed workspace AST symbols into the "code" collection.

## Signature

```rust
pub fn ingest_workspace_ast(&self, symbols: &[ParsedSymbol]) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Ingest parsed workspace AST symbols into the "code" collection.

## Source
Lines 242–319 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
| calls | [format_symbol](/crates/knowledge/src/client/format_symbol.md) |
