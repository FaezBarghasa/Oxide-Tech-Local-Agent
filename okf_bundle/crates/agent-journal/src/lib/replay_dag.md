---
okf_version: "0.2"
type: Function
title: replay_dag
description: "Replay all journal entries for a given `dag_id` in sequence order."
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/replay_dag
language: rust
---

# replay_dag

Replay all journal entries for a given `dag_id` in sequence order.

## Signature

```rust
impl AgentJournal { pub fn replay_dag(&self, dag_id: Uuid) -> Result<Vec<JournalEntry>, JournalError> }
```

## Visibility

- `pub`

## Docstring

Replay all journal entries for a given `dag_id` in sequence order.

Returns entries sorted by `event_seq` ascending — ready for state reconstruction.

## Source
Lines 199–213 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
