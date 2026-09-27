---
okf_version: "0.2"
type: Function
title: get_callers
description: Retrieve all direct callers of a node (1-hop incoming calls edges)
resource: crates/knowledge/src/code_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/code_graph/get_callers_1
language: rust
---

# get_callers

Retrieve all direct callers of a node (1-hop incoming calls edges)

## Signature

```rust
pub fn get_callers(&self, node_id: &str) -> Vec<&CodeNode>
```

## Visibility

- `pub`

## Docstring

Retrieve all direct callers of a node (1-hop incoming calls edges)

## Source
Lines 108–118 in `crates/knowledge/src/code_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [code_graph](/crates/knowledge/src/code_graph.md) |
