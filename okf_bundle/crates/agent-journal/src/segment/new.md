---
okf_version: "0.2"
type: Function
title: new
resource: crates/agent-journal/src/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/segment/new
language: rust
---

# new

## Signature

```rust
impl SegmentJournal { pub fn new(base_dir: impl AsRef<Path>, session_id: Uuid) -> std::io::Result<Self> }
```

## Visibility

- `pub`

## Source
Lines 61–71 in `crates/agent-journal/src/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/agent-journal/src/segment.md) |
