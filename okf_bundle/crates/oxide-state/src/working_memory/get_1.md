---
okf_version: "0.2"
type: Function
title: get
description: "Retrieve a single key from the agent's partition."
resource: crates/oxide-state/src/working_memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/working_memory/get_1
language: rust
---

# get

Retrieve a single key from the agent's partition.

## Signature

```rust
pub fn get(
        &self,
        session_id: Uuid,
        agent_id: &str,
        key: &str,
    ) -> Result<Option<String>, surrealdb::Error>
```

## Visibility

- `pub`

## Docstring

Retrieve a single key from the agent's partition.

## Source
Lines 62–81 in `crates/oxide-state/src/working_memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [working_memory](/crates/oxide-state/src/working_memory.md) |
