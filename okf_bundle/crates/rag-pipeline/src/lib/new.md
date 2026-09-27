---
okf_version: "0.2"
type: Function
title: new
description: "Initialize the RAG pipeline by connecting to Qdrant, initializing FastEmbed, and connecting to SurrealDB."
resource: crates/rag-pipeline/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:44:46Z"
concept_id: crates/rag-pipeline/src/lib/new
language: rust
---

# new

Initialize the RAG pipeline by connecting to Qdrant, initializing FastEmbed, and connecting to SurrealDB.

## Signature

```rust
impl RagPipeline { pub fn new() -> Result<Self, anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Initialize the RAG pipeline by connecting to Qdrant, initializing FastEmbed, and connecting to SurrealDB.

## Source
Lines 52–77 in `crates/rag-pipeline/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/rag-pipeline/src/lib.md) |
