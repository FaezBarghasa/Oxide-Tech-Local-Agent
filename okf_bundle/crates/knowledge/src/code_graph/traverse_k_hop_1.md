---
okf_version: "0.2"
type: Function
title: traverse_k_hop
description: Traverse k-hop neighborhood using Breadth-First Search
resource: crates/knowledge/src/code_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/code_graph/traverse_k_hop_1
language: rust
---

# traverse_k_hop

Traverse k-hop neighborhood using Breadth-First Search

## Signature

```rust
pub fn traverse_k_hop(&self, start_id: &str, k: usize) -> HashSet<String>
```

## Visibility

- `pub`

## Docstring

Traverse k-hop neighborhood using Breadth-First Search

## Source
Lines 134–164 in `crates/knowledge/src/code_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [code_graph](/crates/knowledge/src/code_graph.md) |
