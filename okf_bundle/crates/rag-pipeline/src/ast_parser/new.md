---
okf_version: "0.2"
type: Function
title: new
description: "Create a new `AstParser` watching `root`. Immediately performs an initial scan."
resource: crates/rag-pipeline/src/ast_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:12Z"
concept_id: crates/rag-pipeline/src/ast_parser/new
language: rust
---

# new

Create a new `AstParser` watching `root`. Immediately performs an initial scan.

## Signature

```rust
impl AstParser { pub fn new(root: impl Into<PathBuf>) -> NotifyResult<Self> }
```

## Visibility

- `pub`

## Docstring

Create a new `AstParser` watching `root`. Immediately performs an initial scan.

## Source
Lines 22–58 in `crates/rag-pipeline/src/ast_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast_parser](/crates/rag-pipeline/src/ast_parser.md) |
