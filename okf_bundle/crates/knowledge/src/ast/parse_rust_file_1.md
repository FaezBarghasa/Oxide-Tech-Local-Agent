---
okf_version: "0.2"
type: Function
title: parse_rust_file
resource: crates/knowledge/src/ast.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/ast/parse_rust_file_1
language: rust
---

# parse_rust_file

## Signature

```rust
pub fn parse_rust_file(
        &mut self,
        file_path: &str,
        source: &str,
    ) -> (Vec<CodeGraphNode>, Vec<CodeGraphEdge>)
```

## Visibility

- `pub`

## Source
Lines 102–131 in `crates/knowledge/src/ast.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast](/crates/knowledge/src/ast.md) |
