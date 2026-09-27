---
okf_version: "0.2"
type: Function
title: add_node
resource: crates/optio/src/dag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/optio/src/dag/add_node_1
language: rust
---

# add_node

## Signature

```rust
pub fn add_node(&mut self, id: &str, description: &str, dependencies: Vec<String>)
```

## Visibility

- `pub`

## Source
Lines 35–46 in `crates/optio/src/dag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dag](/crates/optio/src/dag.md) |
