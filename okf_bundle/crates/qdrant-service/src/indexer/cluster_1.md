---
okf_version: "0.2"
type: Function
title: cluster
description: Cluster vectors using exact K-Means.
resource: crates/qdrant-service/src/indexer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:qdrant-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:51:56Z"
concept_id: crates/qdrant-service/src/indexer/cluster_1
language: rust
---

# cluster

Cluster vectors using exact K-Means.

## Signature

```rust
pub fn cluster(&self, vectors: &[Vec<f32>]) -> Vec<usize>
```

## Visibility

- `pub`

## Docstring

Cluster vectors using exact K-Means.
Uses IO-aware execution paths to partition large files/embeddings.

## Source
Lines 13–77 in `crates/qdrant-service/src/indexer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [indexer](/crates/qdrant-service/src/indexer.md) |
| calls | [euclidean_distance](/crates/qdrant-service/src/indexer/euclidean_distance.md) |
