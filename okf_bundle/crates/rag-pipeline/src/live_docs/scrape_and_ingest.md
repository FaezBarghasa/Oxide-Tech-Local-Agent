---
okf_version: "0.2"
type: Function
title: scrape_and_ingest
description: Scrape the latest docs for the given crate name and ingest.
resource: crates/rag-pipeline/src/live_docs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:20:28Z"
concept_id: crates/rag-pipeline/src/live_docs/scrape_and_ingest
language: rust
---

# scrape_and_ingest

Scrape the latest docs for the given crate name and ingest.

## Signature

```rust
impl LiveDocsScraper { pub fn scrape_and_ingest(&self, crate_name: &str, version: &str) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Scrape the latest docs for the given crate name and ingest.
For simplicity, we delegate to `RagPipeline::ingest_crate_docs` with version "latest".

## Source
Lines 19–25 in `crates/rag-pipeline/src/live_docs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [live_docs](/crates/rag-pipeline/src/live_docs.md) |
