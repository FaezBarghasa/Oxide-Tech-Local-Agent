---
okf_version: "0.2"
type: Class
title: CodeEdge
description: A directed edge in the code graph
resource: crates/knowledge/src/code_graph.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/code_graph/CodeEdge
language: rust
---

# CodeEdge

A directed edge in the code graph

## Signature

```rust
pub struct CodeEdge
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

A directed edge in the code graph
[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]

## Methods

- `from_id`
- `to_id`
- `edge_type`
- `weight`

## Source
Lines 58–63 in `crates/knowledge/src/code_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [code_graph](/crates/knowledge/src/code_graph.md) |
