---
okf_version: "0.2"
type: Function
title: ingest_crate_docs
description: "Crawl and ingest docs.rs pages for a specific crate and version into \"documentation\" collection."
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/ingest_crate_docs
language: rust
---

# ingest_crate_docs

Crawl and ingest docs.rs pages for a specific crate and version into "documentation" collection.

## Signature

```rust
impl KnowledgeClient { pub fn ingest_crate_docs(&self, crate_name: &str, version: &str) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Crawl and ingest docs.rs pages for a specific crate and version into "documentation" collection.

## Source
Lines 63–181 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
| calls | [extract_text_from_html](/crates/knowledge/src/client/extract_text_from_html.md) |
| calls | [chunk_text](/crates/knowledge/src/client/chunk_text.md) |
