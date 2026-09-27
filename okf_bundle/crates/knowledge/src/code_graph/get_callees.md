---
okf_version: "0.2"
type: Function
title: get_callees
description: Retrieve all direct callees of a node (1-hop outgoing calls edges)
resource: crates/knowledge/src/code_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/code_graph/get_callees
language: rust
---

# get_callees

Retrieve all direct callees of a node (1-hop outgoing calls edges)

## Signature

```rust
impl MultiModalCodeGraph { pub fn get_callees(&self, node_id: &str) -> Vec<&CodeNode> }
```

## Visibility

- `pub`

## Docstring

Retrieve all direct callees of a node (1-hop outgoing calls edges)

## Source
Lines 121–131 in `crates/knowledge/src/code_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [code_graph](/crates/knowledge/src/code_graph.md) |
