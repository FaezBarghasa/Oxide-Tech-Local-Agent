---
okf_version: "0.2"
type: Function
title: parse_file
resource: crates/tree-sitter-service/src/parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:16:24Z"
concept_id: crates/tree-sitter-service/src/parser/parse_file
language: rust
---

# parse_file

## Signature

```rust
pub fn parse_file(content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String>
```

## Visibility

- `pub`

## Source
Lines 5–8 in `crates/tree-sitter-service/src/parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/tree-sitter-service/src/parser.md) |
| calls | [parse_file_with_language](/crates/tree-sitter-service/src/parser/parse_file_with_language.md) |
| called_by | [test_c_cpp_parsing](/crates/tree-sitter-service/src/languages/test_c_cpp_parsing.md) |
| called_by | [test_openscad_parsing](/crates/tree-sitter-service/src/languages/test_openscad_parsing.md) |
| called_by | [test_python_parsing](/crates/tree-sitter-service/src/languages/test_python_parsing.md) |
| called_by | [test_rust_parsing](/crates/tree-sitter-service/src/languages/test_rust_parsing.md) |
| called_by | [test_systemverilog_parsing](/crates/tree-sitter-service/src/languages/test_systemverilog_parsing.md) |
