---
okf_version: "0.2"
type: Function
title: append
description: Append a new journal event.
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/append
language: rust
---

# append

Append a new journal event.

## Signature

```rust
impl AgentJournal { pub fn append(&self, event: JournalEvent) -> Result<u64, JournalError> }
```

## Visibility

- `pub`

## Docstring

Append a new journal event.

Returns the `event_seq` assigned to this entry.

## Source
Lines 181–194 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
