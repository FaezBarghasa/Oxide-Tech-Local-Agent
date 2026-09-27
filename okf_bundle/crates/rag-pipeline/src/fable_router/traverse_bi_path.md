---
okf_version: "0.2"
type: Function
title: traverse_bi_path
description: "Run the Bi-Path traversal strategy: Balance tree hierarchy with SIMD vector matching"
resource: crates/rag-pipeline/src/fable_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/rag-pipeline/src/fable_router/traverse_bi_path
language: rust
---

# traverse_bi_path

Run the Bi-Path traversal strategy: Balance tree hierarchy with SIMD vector matching

## Signature

```rust
impl FableForest { pub fn traverse_bi_path(
        &self,
        query_vector: &[f32],
        seed_node_id: &str,
        max_results: usize,
    ) -> Vec<String> }
```

## Visibility

- `pub`

## Docstring

Run the Bi-Path traversal strategy: Balance tree hierarchy with SIMD vector matching

## Source
Lines 58–99 in `crates/rag-pipeline/src/fable_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fable_router](/crates/rag-pipeline/src/fable_router.md) |
