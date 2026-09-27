---
okf_version: "0.2"
type: Function
title: traverse_nodes
resource: crates/tree-sitter-service/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:16:24Z"
concept_id: crates/tree-sitter-service/src/parser/traverse_nodes
language: rust
---

# traverse_nodes

## Signature

```rust
fn traverse_nodes(node: Node, content: &str, file_path: &str, symbols: &mut Vec<ParsedSymbol>)
```

## Source
Lines 58–355 in `crates/tree-sitter-service/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/tree-sitter-service/src/parser.md) |
| calls | [get_previous_doc_comments](/crates/tree-sitter-service/src/parser/get_previous_doc_comments.md) |
| called_by | [parse_rust_content](/crates/tree-sitter-service/src/parser/parse_rust_content.md) |
