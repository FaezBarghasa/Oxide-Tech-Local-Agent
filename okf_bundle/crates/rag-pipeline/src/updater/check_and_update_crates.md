---
okf_version: "0.2"
type: Function
title: check_and_update_crates
description: Iterates through the watchlist of crates. If a crate is missing from SurrealDB
resource: crates/rag-pipeline/src/updater.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/rag-pipeline/src/updater/check_and_update_crates
language: rust
---

# check_and_update_crates

Iterates through the watchlist of crates. If a crate is missing from SurrealDB

## Signature

```rust
pub fn check_and_update_crates(
    watchlist: &[String],
    pipeline: &RagPipeline,
) -> Result<(), anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Iterates through the watchlist of crates. If a crate is missing from SurrealDB
or has a newer version on crates.io, we fetch and index its docs.rs pages.

## Source
Lines 59–152 in `crates/rag-pipeline/src/updater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updater](/crates/rag-pipeline/src/updater.md) |
| calls | [fetch_latest_crates_io_version](/crates/rag-pipeline/src/updater/fetch_latest_crates_io_version.md) |
| called_by | [main](/crates/api/src/main/main.md) |
| called_by | [handle_rag_update](/crates/api/src/routes/agent/handle_rag_update.md) |
