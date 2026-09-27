---
okf_version: "0.2"
type: Function
title: run_tree_sitter_parse
resource: crates/api/src/routes/tree_sitter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/tree_sitter/run_tree_sitter_parse
language: rust
---

# run_tree_sitter_parse

## Signature

```rust
pub fn run_tree_sitter_parse(req: ParseRequest) -> Result<serde_json::Value, String>
```

## Visibility

- `pub`

## Source
Lines 33–78 in `crates/api/src/routes/tree_sitter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_sitter](/crates/api/src/routes/tree_sitter.md) |
| calls | [get_rs_files](/crates/api/src/routes/tree_sitter/get_rs_files.md) |
| called_by | [handle_tree_sitter_parse](/crates/api/src/routes/tree_sitter/handle_tree_sitter_parse.md) |
