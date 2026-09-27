---
okf_version: "0.2"
type: Function
title: traverse_tree
resource: crates/knowledge/src/ast.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/ast/traverse_tree_1
language: rust
---

# traverse_tree

## Signature

```rust
fn traverse_tree(
        &self,
        node: Node,
        source: &str,
        file_path: &str,
        parent_id: &str,
        nodes: &mut Vec<CodeGraphNode>,
        edges: &mut Vec<CodeGraphEdge>,
    )
```

## Source
Lines 133–198 in `crates/knowledge/src/ast.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast](/crates/knowledge/src/ast.md) |
