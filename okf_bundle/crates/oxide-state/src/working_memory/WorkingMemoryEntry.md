---
okf_version: "0.2"
type: Class
title: WorkingMemoryEntry
description: "An entry in an agent's working memory buffer."
resource: crates/oxide-state/src/working_memory.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/working_memory/WorkingMemoryEntry
language: rust
---

# WorkingMemoryEntry

An entry in an agent's working memory buffer.

## Signature

```rust
pub struct WorkingMemoryEntry
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, SurrealValue)`

## Visibility

- `pub`

## Docstring

An entry in an agent's working memory buffer.
[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]

## Methods

- `id`
- `session_id`
- `agent_id`
- `key`
- `value`
- `updated_at`

## Source
Lines 16–24 in `crates/oxide-state/src/working_memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [working_memory](/crates/oxide-state/src/working_memory.md) |
