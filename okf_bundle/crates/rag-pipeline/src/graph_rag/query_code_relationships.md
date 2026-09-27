---
okf_version: "0.2"
type: Function
title: query_code_relationships
description: Query the SurrealDB graph for symbol dependencies or code relationships
resource: crates/rag-pipeline/src/graph_rag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:41:07Z"
concept_id: crates/rag-pipeline/src/graph_rag/query_code_relationships
language: rust
---

# query_code_relationships

Query the SurrealDB graph for symbol dependencies or code relationships

## Signature

```rust
impl GraphRagEngine { pub fn query_code_relationships(
        &self,
        symbol_name: &str,
    ) -> Result<Vec<Value>, anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Query the SurrealDB graph for symbol dependencies or code relationships

## Source
Lines 15–27 in `crates/rag-pipeline/src/graph_rag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graph_rag](/crates/rag-pipeline/src/graph_rag.md) |
