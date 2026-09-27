---
okf_version: "0.2"
type: Function
title: ingest_github_releases
description: "Ingest a GitHub repository or organization into the \"documentation\" collection."
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/ingest_github_releases_1
language: rust
---

# ingest_github_releases

Ingest a GitHub repository or organization into the "documentation" collection.

## Signature

```rust
pub fn ingest_github_releases(&self, repo_url: &str) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Ingest a GitHub repository or organization into the "documentation" collection.

## Source
Lines 490–575 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
| calls | [parse_feed](/crates/knowledge/src/rss/parse_feed.md) |
| calls | [chunk_text](/crates/knowledge/src/client/chunk_text.md) |
| calls | [extract_text_from_html](/crates/knowledge/src/client/extract_text_from_html.md) |
