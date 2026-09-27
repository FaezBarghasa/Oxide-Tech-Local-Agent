---
okf_version: "0.2"
type: Function
title: ingest_custom_url
description: "Scrape and ingest arbitrary web page into the \"documentation\" collection."
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/ingest_custom_url
language: rust
---

# ingest_custom_url

Scrape and ingest arbitrary web page into the "documentation" collection.

## Signature

```rust
impl KnowledgeClient { pub fn ingest_custom_url(&self, url: &str) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Scrape and ingest arbitrary web page into the "documentation" collection.

## Source
Lines 578–618 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
| calls | [extract_text_from_html](/crates/knowledge/src/client/extract_text_from_html.md) |
| calls | [chunk_text](/crates/knowledge/src/client/chunk_text.md) |
