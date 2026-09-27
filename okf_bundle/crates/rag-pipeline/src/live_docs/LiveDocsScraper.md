---
okf_version: "0.2"
type: Class
title: LiveDocsScraper
description: Helper for scraping docs.rs for a crate and ingesting it into the RAG pipeline.
resource: crates/rag-pipeline/src/live_docs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:20:28Z"
concept_id: crates/rag-pipeline/src/live_docs/LiveDocsScraper
language: rust
---

# LiveDocsScraper

Helper for scraping docs.rs for a crate and ingesting it into the RAG pipeline.

## Signature

```rust
pub struct LiveDocsScraper
```

## Visibility

- `pub`

## Docstring

Helper for scraping docs.rs for a crate and ingesting it into the RAG pipeline.
This is primarily used by the MCP `live_docs_scrape` tool.

## Methods

- `pipeline`

## Source
Lines 8–10 in `crates/rag-pipeline/src/live_docs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [live_docs](/crates/rag-pipeline/src/live_docs.md) |
