---
okf_version: "0.2"
type: Function
title: extract_text_from_html
description: ── Text Extraction Helper ────────────────────────────────────────────────────
resource: crates/rag-pipeline/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:44:46Z"
concept_id: crates/rag-pipeline/src/lib/extract_text_from_html
language: rust
---

# extract_text_from_html

── Text Extraction Helper ────────────────────────────────────────────────────

## Signature

```rust
fn extract_text_from_html(html: &str) -> String
```

## Docstring

── Text Extraction Helper ────────────────────────────────────────────────────

## Source
Lines 370–382 in `crates/rag-pipeline/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/rag-pipeline/src/lib.md) |
| called_by | [ingest_crate_docs](/crates/rag-pipeline/src/lib/ingest_crate_docs.md) |
