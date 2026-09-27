---
okf_version: "0.2"
type: Function
title: snapshot
description: Retrieve a snapshot of the current symbol table.
resource: crates/rag-pipeline/src/ast_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:12Z"
concept_id: crates/rag-pipeline/src/ast_parser/snapshot_1
language: rust
---

# snapshot

Retrieve a snapshot of the current symbol table.

## Signature

```rust
pub fn snapshot(&self) -> HashMap<PathBuf, Vec<String>>
```

## Visibility

- `pub`

## Docstring

Retrieve a snapshot of the current symbol table.

## Source
Lines 84–86 in `crates/rag-pipeline/src/ast_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast_parser](/crates/rag-pipeline/src/ast_parser.md) |
