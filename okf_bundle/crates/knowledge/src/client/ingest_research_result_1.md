---
okf_version: "0.2"
type: Function
title: ingest_research_result
description: "Ingest research results from External Research & Perception Layer with strict provenance"
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/ingest_research_result_1
language: rust
---

# ingest_research_result

Ingest research results from External Research & Perception Layer with strict provenance

## Signature

```rust
pub fn ingest_research_result(
        &self,
        url: &str,
        text: &str,
        engine: &str,
        confidence_score: f32,
        title: Option<&str>,
    ) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Ingest research results from External Research & Perception Layer with strict provenance

## Source
Lines 184–239 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
| calls | [chunk_text](/crates/knowledge/src/client/chunk_text.md) |
