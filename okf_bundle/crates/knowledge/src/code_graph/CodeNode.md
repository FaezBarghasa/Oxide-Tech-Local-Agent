---
okf_version: "0.2"
type: Class
title: CodeNode
description: A node in the code graph representing an AST symbol or state construct
resource: crates/knowledge/src/code_graph.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/code_graph/CodeNode
language: rust
---

# CodeNode

A node in the code graph representing an AST symbol or state construct

## Signature

```rust
pub struct CodeNode
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

A node in the code graph representing an AST symbol or state construct
[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]

## Methods

- `id`
- `name`
- `node_type`
- `file_path`
- `span_start`
- `span_end`
- `signature`
- `doc_comment`
- `vector_id`

## Source
Lines 23–33 in `crates/knowledge/src/code_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [code_graph](/crates/knowledge/src/code_graph.md) |
