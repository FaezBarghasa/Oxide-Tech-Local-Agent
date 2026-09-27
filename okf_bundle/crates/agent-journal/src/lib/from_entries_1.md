---
okf_version: "0.2"
type: Function
title: from_entries
description: Build reconstructed state from a sorted sequence of journal entries.
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/from_entries_1
language: rust
---

# from_entries

Build reconstructed state from a sorted sequence of journal entries.

## Signature

```rust
pub fn from_entries(entries: &[JournalEntry]) -> Result<Self, JournalError>
```

## Visibility

- `pub`

## Docstring

Build reconstructed state from a sorted sequence of journal entries.

## Source
Lines 261–310 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
