---
okf_version: "0.2"
type: Function
title: parse_rust_content
resource: crates/tree-sitter-service/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:16:24Z"
concept_id: crates/tree-sitter-service/src/parser/parse_rust_content
language: rust
---

# parse_rust_content

## Signature

```rust
pub(crate) fn parse_rust_content(content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String>
```

## Visibility

- `pub(crate)`

## Source
Lines 19–29 in `crates/tree-sitter-service/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/tree-sitter-service/src/parser.md) |
| calls | [traverse_nodes](/crates/tree-sitter-service/src/parser/traverse_nodes.md) |
| called_by | [parse](/crates/tree-sitter-service/src/languages/parse.md) |
