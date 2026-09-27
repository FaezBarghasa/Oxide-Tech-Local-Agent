---
okf_version: "0.2"
type: Function
title: search
description: Dense vector search over the collection.
resource: crates/rag-pipeline/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:44:46Z"
concept_id: crates/rag-pipeline/src/lib/search
language: rust
---

# search

Dense vector search over the collection.

## Signature

```rust
impl RagPipeline { pub fn search(&self, query: &str, top_k: usize) -> Result<Vec<RagChunk>, anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Dense vector search over the collection.

## Source
Lines 296–365 in `crates/rag-pipeline/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/rag-pipeline/src/lib.md) |
