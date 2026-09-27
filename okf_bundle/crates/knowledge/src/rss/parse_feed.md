---
okf_version: "0.2"
type: Function
title: parse_feed
resource: crates/knowledge/src/rss.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/rss/parse_feed
language: rust
---

# parse_feed

## Signature

```rust
pub fn parse_feed(content: &str) -> Vec<FeedEntry>
```

## Visibility

- `pub`

## Source
Lines 13–104 in `crates/knowledge/src/rss.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rss](/crates/knowledge/src/rss.md) |
| called_by | [ingest_github_releases](/crates/knowledge/src/client/ingest_github_releases.md) |
| called_by | [ingest_rss_feed](/crates/knowledge/src/client/ingest_rss_feed.md) |
| called_by | [test_parse_atom_feed](/crates/knowledge/src/lib/test_parse_atom_feed.md) |
| called_by | [test_parse_rss_feed](/crates/knowledge/src/lib/test_parse_rss_feed.md) |
| called_by | [run_update_cycle](/crates/scheduler/src/lib/run_update_cycle.md) |
