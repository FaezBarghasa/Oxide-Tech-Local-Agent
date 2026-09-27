---
okf_version: "0.2"
type: Function
title: journal_append
description: "Convenience: append a journal event if a journal is attached."
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/journal_append_1
language: rust
---

# journal_append

Convenience: append a journal event if a journal is attached.

## Signature

```rust
pub fn journal_append(&self, event: JournalEvent)
```

## Visibility

- `pub`

## Docstring

Convenience: append a journal event if a journal is attached.

## Source
Lines 240–246 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
