---
okf_version: "0.2"
type: Class
title: JournalEntry
description: "A single immutable row in the `agent_journal` SurrealDB table."
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/JournalEntry
language: rust
---

# JournalEntry

A single immutable row in the `agent_journal` SurrealDB table.

## Signature

```rust
pub struct JournalEntry
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, SurrealValue)`

## Visibility

- `pub`

## Docstring

A single immutable row in the `agent_journal` SurrealDB table.
[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]

## Methods

- `id`
- `event_seq`
- `event`
- `recorded_at`

## Source
Lines 146–156 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
