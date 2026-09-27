---
okf_version: "0.2"
type: Class
title: SegmentJournal
description: "Append-only segment file logger (`seg-*.rjnl`)"
resource: crates/agent-journal/src/segment.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/segment/SegmentJournal
language: rust
---

# SegmentJournal

Append-only segment file logger (`seg-*.rjnl`)

## Signature

```rust
pub struct SegmentJournal
```

## Visibility

- `pub`

## Docstring

Append-only segment file logger (`seg-*.rjnl`)

## Methods

- `session_id`
- `journal_dir`
- `current_seq`
- `last_hash`

## Source
Lines 53–58 in `crates/agent-journal/src/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/agent-journal/src/segment.md) |
