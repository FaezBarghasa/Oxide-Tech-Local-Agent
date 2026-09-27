---
okf_version: "0.2"
type: Function
title: consolidate_and_compress
description: Consolidates transient working memory entries into a compressed persistent knowledge snapshot
resource: crates/oxide-state/src/working_memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/working_memory/consolidate_and_compress
language: rust
---

# consolidate_and_compress

Consolidates transient working memory entries into a compressed persistent knowledge snapshot

## Signature

```rust
impl WorkingMemoryManager { pub fn consolidate_and_compress(
        &self,
        session_id: Uuid,
        agent_id: &str,
    ) -> Result<String, surrealdb::Error> }
```

## Visibility

- `pub`

## Docstring

Consolidates transient working memory entries into a compressed persistent knowledge snapshot

## Source
Lines 120–152 in `crates/oxide-state/src/working_memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [working_memory](/crates/oxide-state/src/working_memory.md) |
