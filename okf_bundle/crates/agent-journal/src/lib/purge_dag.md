---
okf_version: "0.2"
type: Function
title: purge_dag
description: Delete all journal entries for a completed DAG (optional cleanup after archival).
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/purge_dag
language: rust
---

# purge_dag

Delete all journal entries for a completed DAG (optional cleanup after archival).

## Signature

```rust
impl AgentJournal { pub fn purge_dag(&self, dag_id: Uuid) -> Result<usize, JournalError> }
```

## Visibility

- `pub`

## Docstring

Delete all journal entries for a completed DAG (optional cleanup after archival).

## Source
Lines 228–238 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
