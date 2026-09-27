---
okf_version: "0.2"
type: Function
title: list_symbols
description: "[tool(description = \"List all parsed symbols in a file using tree-sitter\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/list_symbols_1
language: rust
---

# list_symbols

[tool(description = "List all parsed symbols in a file using tree-sitter")]

## Signature

```rust
fn list_symbols(
        &self,
        Parameters(input): Parameters<ListSymbolsInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "List all parsed symbols in a file using tree-sitter")`

## Docstring

[tool(description = "List all parsed symbols in a file using tree-sitter")]

## Source
Lines 480–508 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
