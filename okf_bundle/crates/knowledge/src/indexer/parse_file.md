---
okf_version: "0.2"
type: Function
title: parse_file
resource: crates/knowledge/src/indexer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/indexer/parse_file
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
Lines 5–15 in `crates/knowledge/src/indexer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [indexer](/crates/knowledge/src/indexer.md) |
| calls | [traverse_nodes](/crates/knowledge/src/indexer/traverse_nodes.md) |
