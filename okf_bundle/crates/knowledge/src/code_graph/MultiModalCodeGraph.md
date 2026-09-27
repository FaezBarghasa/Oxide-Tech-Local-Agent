---
okf_version: "0.2"
type: Class
title: MultiModalCodeGraph
description: "Multi-Modal Code Graph: Topological mapping of AST, Call, Dependency, and Data Flow graphs"
resource: crates/knowledge/src/code_graph.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/code_graph/MultiModalCodeGraph
language: rust
---

# MultiModalCodeGraph

Multi-Modal Code Graph: Topological mapping of AST, Call, Dependency, and Data Flow graphs

## Signature

```rust
pub struct MultiModalCodeGraph
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, Default)`

## Visibility

- `pub`

## Docstring

Multi-Modal Code Graph: Topological mapping of AST, Call, Dependency, and Data Flow graphs
[derive(Debug, Serialize, Deserialize, Clone, Default)]

## Methods

- `nodes`
- `outgoing_edges`
- `incoming_edges`

## Source
Lines 67–71 in `crates/knowledge/src/code_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [code_graph](/crates/knowledge/src/code_graph.md) |
