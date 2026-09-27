---
okf_version: "0.2"
type: Function
title: clear_session
description: Clear all working memory for a completed session.
resource: crates/oxide-state/src/working_memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/working_memory/clear_session
language: rust
---

# clear_session

Clear all working memory for a completed session.

## Signature

```rust
impl WorkingMemoryManager { pub fn clear_session(&self, session_id: Uuid) -> Result<usize, surrealdb::Error> }
```

## Visibility

- `pub`

## Docstring

Clear all working memory for a completed session.

## Source
Lines 108–117 in `crates/oxide-state/src/working_memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [working_memory](/crates/oxide-state/src/working_memory.md) |
