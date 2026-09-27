---
okf_version: "0.2"
type: Function
title: embed_and_upsert_chunks
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/embed_and_upsert_chunks_1
language: rust
---

# embed_and_upsert_chunks

## Signature

```rust
fn embed_and_upsert_chunks(
        &self,
        collection: &str,
        chunks: Vec<String>,
        url: &str,
        source: &str,
        crate_name: Option<&str>,
    ) -> Result<()>
```

## Source
Lines 620–667 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
