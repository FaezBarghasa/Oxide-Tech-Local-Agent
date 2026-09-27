---
okf_version: "0.2"
type: Function
title: set
description: "Store or update a key-value pair for a specific (session, agent) partition."
resource: crates/oxide-state/src/working_memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/working_memory/set
language: rust
---

# set

Store or update a key-value pair for a specific (session, agent) partition.

## Signature

```rust
impl WorkingMemoryManager { pub fn set(
        &self,
        session_id: Uuid,
        agent_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), surrealdb::Error> }
```

## Visibility

- `pub`

## Docstring

Store or update a key-value pair for a specific (session, agent) partition.

## Source
Lines 38–59 in `crates/oxide-state/src/working_memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [working_memory](/crates/oxide-state/src/working_memory.md) |
