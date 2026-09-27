---
okf_version: "0.2"
type: Function
title: tail
description: "Return the last `N` journal entries across all DAGs (for live monitoring)."
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/tail_1
language: rust
---

# tail

Return the last `N` journal entries across all DAGs (for live monitoring).

## Signature

```rust
pub fn tail(&self, n: usize) -> Result<Vec<JournalEntry>, JournalError>
```

## Visibility

- `pub`

## Docstring

Return the last `N` journal entries across all DAGs (for live monitoring).

## Source
Lines 216–225 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
