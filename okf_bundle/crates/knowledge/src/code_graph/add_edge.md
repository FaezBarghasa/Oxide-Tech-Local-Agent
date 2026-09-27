---
okf_version: "0.2"
type: Function
title: add_edge
description: Add a directed edge between two nodes
resource: crates/knowledge/src/code_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/code_graph/add_edge
language: rust
---

# add_edge

Add a directed edge between two nodes

## Signature

```rust
impl MultiModalCodeGraph { pub fn add_edge(&mut self, from_id: &str, to_id: &str, edge_type: CodeEdgeType, weight: f32) }
```

## Visibility

- `pub`

## Docstring

Add a directed edge between two nodes

## Source
Lines 88–105 in `crates/knowledge/src/code_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [code_graph](/crates/knowledge/src/code_graph.md) |
