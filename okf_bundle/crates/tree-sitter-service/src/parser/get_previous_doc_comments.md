---
okf_version: "0.2"
type: Function
title: get_previous_doc_comments
resource: crates/tree-sitter-service/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:16:24Z"
concept_id: crates/tree-sitter-service/src/parser/get_previous_doc_comments
language: rust
---

# get_previous_doc_comments

## Signature

```rust
fn get_previous_doc_comments(node: Node, content: &str) -> Option<String>
```

## Source
Lines 32–56 in `crates/tree-sitter-service/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/tree-sitter-service/src/parser.md) |
| called_by | [traverse_nodes](/crates/tree-sitter-service/src/parser/traverse_nodes.md) |
