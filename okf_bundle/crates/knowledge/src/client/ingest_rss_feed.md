---
okf_version: "0.2"
type: Function
title: ingest_rss_feed
description: "Ingest named RSS/Atom feed into \"news_raw\" collection."
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/ingest_rss_feed
language: rust
---

# ingest_rss_feed

Ingest named RSS/Atom feed into "news_raw" collection.

## Signature

```rust
impl KnowledgeClient { pub fn ingest_rss_feed(&self, feed_url: &str, source_name: &str) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Ingest named RSS/Atom feed into "news_raw" collection.

## Source
Lines 402–487 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
| calls | [parse_feed](/crates/knowledge/src/rss/parse_feed.md) |
| calls | [chunk_text](/crates/knowledge/src/client/chunk_text.md) |
