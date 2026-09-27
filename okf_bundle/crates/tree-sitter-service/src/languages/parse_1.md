---
okf_version: "0.2"
type: Function
title: parse
resource: crates/tree-sitter-service/src/languages.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:26:05Z"
concept_id: crates/tree-sitter-service/src/languages/parse_1
language: rust
---

# parse

## Signature

```rust
fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String>
```

## Source
Lines 233–235 in `crates/tree-sitter-service/src/languages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [languages](/crates/tree-sitter-service/src/languages.md) |
| calls | [parse_rust_content](/crates/tree-sitter-service/src/parser/parse_rust_content.md) |
