---
okf_version: "0.2"
type: Function
title: qdrant_search
description: "[tool(description = \"Semantic search over embedded Rust crate docs and workspace AST\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/qdrant_search_1
language: rust
---

# qdrant_search

[tool(description = "Semantic search over embedded Rust crate docs and workspace AST")]

## Signature

```rust
fn qdrant_search(
        &self,
        Parameters(input): Parameters<QdrantSearchInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Semantic search over embedded Rust crate docs and workspace AST")`

## Docstring

[tool(description = "Semantic search over embedded Rust crate docs and workspace AST")]

## Source
Lines 397–422 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
