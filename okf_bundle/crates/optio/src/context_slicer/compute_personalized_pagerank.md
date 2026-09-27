---
okf_version: "0.2"
type: Function
title: compute_personalized_pagerank
description: Compute Personalized PageRank (PPR) starting from a target node
resource: crates/optio/src/context_slicer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:13:37Z"
concept_id: crates/optio/src/context_slicer/compute_personalized_pagerank
language: rust
---

# compute_personalized_pagerank

Compute Personalized PageRank (PPR) starting from a target node

## Signature

```rust
impl GraphContextSlicer { pub fn compute_personalized_pagerank(
        &self,
        target_id: &str,
        damping: f64,
        max_iters: usize,
        top_k: usize,
    ) -> Vec<SubgraphSlice> }
```

## Visibility

- `pub`

## Docstring

Compute Personalized PageRank (PPR) starting from a target node
Returns the top_k most relevant nodes, achieving 60-80% token reduction.

## Source
Lines 62–162 in `crates/optio/src/context_slicer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_slicer](/crates/optio/src/context_slicer.md) |
