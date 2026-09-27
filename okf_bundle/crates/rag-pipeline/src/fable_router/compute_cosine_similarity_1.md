---
okf_version: "0.2"
type: Function
title: compute_cosine_similarity
description: Fast SIMD / AVX-512 cosine similarity matching between query and node vector
resource: crates/rag-pipeline/src/fable_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/rag-pipeline/src/fable_router/compute_cosine_similarity_1
language: rust
---

# compute_cosine_similarity

Fast SIMD / AVX-512 cosine similarity matching between query and node vector

## Signature

```rust
pub fn compute_cosine_similarity(a: &[f32], b: &[f32]) -> f32
```

## Decorators

- `inline(always)`

## Visibility

- `pub`

## Docstring

Fast SIMD / AVX-512 cosine similarity matching between query and node vector
[inline(always)]

## Source
Lines 34–55 in `crates/rag-pipeline/src/fable_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fable_router](/crates/rag-pipeline/src/fable_router.md) |
