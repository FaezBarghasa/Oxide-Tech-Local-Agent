---
okf_version: "0.2"
type: Function
title: extract_python_doc
resource: crates/tree-sitter-service/src/languages.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:tree-sitter-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T02:26:05Z"
concept_id: crates/tree-sitter-service/src/languages/extract_python_doc
language: rust
---

# extract_python_doc

## Signature

```rust
fn extract_python_doc(lines: &[&str], start_idx: usize) -> Option<String>
```

## Source
Lines 324–353 in `crates/tree-sitter-service/src/languages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [languages](/crates/tree-sitter-service/src/languages.md) |
| called_by | [parse](/crates/tree-sitter-service/src/languages/parse.md) |
