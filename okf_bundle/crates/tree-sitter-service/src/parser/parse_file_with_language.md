---
okf_version: "0.2"
type: Function
title: parse_file_with_language
resource: crates/tree-sitter-service/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:16:24Z"
concept_id: crates/tree-sitter-service/src/parser/parse_file_with_language
language: rust
---

# parse_file_with_language

## Signature

```rust
pub fn parse_file_with_language(
    content: &str,
    file_path: &str,
    lang: crate::languages::LanguageId,
) -> Result<Vec<ParsedSymbol>, String>
```

## Visibility

- `pub`

## Source
Lines 10–17 in `crates/tree-sitter-service/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/tree-sitter-service/src/parser.md) |
| calls | [get_parser](/crates/tree-sitter-service/src/languages/get_parser.md) |
| called_by | [parse_file](/crates/tree-sitter-service/src/parser/parse_file.md) |
