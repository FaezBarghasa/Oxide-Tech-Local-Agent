---
okf_version: "0.2"
type: Function
title: search
description: Generic semantic search over any collection.
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/search_1
language: rust
---

# search

Generic semantic search over any collection.

## Signature

```rust
pub fn search(
        &self,
        collection: &str,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<EiosChunk>>
```

## Visibility

- `pub`

## Docstring

Generic semantic search over any collection.

## Source
Lines 322–399 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
