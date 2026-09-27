---
okf_version: "0.2"
type: Function
title: ingest_crate_docs
description: Crawl and ingest docs.rs pages for a specific crate and version.
resource: crates/rag-pipeline/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:44:46Z"
concept_id: crates/rag-pipeline/src/lib/ingest_crate_docs
language: rust
---

# ingest_crate_docs

Crawl and ingest docs.rs pages for a specific crate and version.

## Signature

```rust
impl RagPipeline { pub fn ingest_crate_docs(
        &self,
        crate_name: &str,
        version: &str,
    ) -> Result<(), anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Crawl and ingest docs.rs pages for a specific crate and version.

## Source
Lines 99–213 in `crates/rag-pipeline/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/rag-pipeline/src/lib.md) |
| calls | [extract_text_from_html](/crates/rag-pipeline/src/lib/extract_text_from_html.md) |
| calls | [chunk_text](/crates/rag-pipeline/src/lib/chunk_text.md) |
