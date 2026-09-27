---
okf_version: "0.2"
type: Function
title: get_parser
resource: crates/tree-sitter-service/src/languages.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:26:05Z"
concept_id: crates/tree-sitter-service/src/languages/get_parser
language: rust
---

# get_parser

## Signature

```rust
pub fn get_parser(lang: LanguageId) -> Box<dyn LanguageParser>
```

## Visibility

- `pub`

## Source
Lines 738–747 in `crates/tree-sitter-service/src/languages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [languages](/crates/tree-sitter-service/src/languages.md) |
| called_by | [parse_file_with_language](/crates/tree-sitter-service/src/parser/parse_file_with_language.md) |
