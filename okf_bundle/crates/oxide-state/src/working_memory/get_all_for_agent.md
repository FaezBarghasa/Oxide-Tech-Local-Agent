---
okf_version: "0.2"
type: Function
title: get_all_for_agent
description: Retrieve all keys and values for an agent within the current session.
resource: crates/oxide-state/src/working_memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/working_memory/get_all_for_agent
language: rust
---

# get_all_for_agent

Retrieve all keys and values for an agent within the current session.

## Signature

```rust
impl WorkingMemoryManager { pub fn get_all_for_agent(
        &self,
        session_id: Uuid,
        agent_id: &str,
    ) -> Result<HashMap<String, String>, surrealdb::Error> }
```

## Visibility

- `pub`

## Docstring

Retrieve all keys and values for an agent within the current session.

## Source
Lines 84–105 in `crates/oxide-state/src/working_memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [working_memory](/crates/oxide-state/src/working_memory.md) |
