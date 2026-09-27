---
okf_version: "0.2"
type: Function
title: fetch_latest_crates_io_version
description: Fetch the latest max stable version of a crate from the crates.io API.
resource: crates/rag-pipeline/src/updater.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/rag-pipeline/src/updater/fetch_latest_crates_io_version
language: rust
---

# fetch_latest_crates_io_version

Fetch the latest max stable version of a crate from the crates.io API.

## Signature

```rust
pub fn fetch_latest_crates_io_version(
    client: &reqwest::Client,
    crate_name: &str,
) -> Result<String, anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Fetch the latest max stable version of a crate from the crates.io API.

## Source
Lines 25–55 in `crates/rag-pipeline/src/updater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updater](/crates/rag-pipeline/src/updater.md) |
| called_by | [fetch_crate_docs](/crates/oxide-mcp/src/handler/fetch_crate_docs.md) |
| called_by | [check_and_update_crates](/crates/rag-pipeline/src/updater/check_and_update_crates.md) |
