---
okf_version: "0.2"
type: Function
title: append
resource: crates/agent-journal/src/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/segment/append_1
language: rust
---

# append

## Signature

```rust
pub fn append(
        &self,
        dtx_id: DtxId,
        kind: EventKind,
        payload: Vec<u8>,
    ) -> Result<SegmentEvent, SegmentJournalError>
```

## Visibility

- `pub`

## Source
Lines 73–111 in `crates/agent-journal/src/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/agent-journal/src/segment.md) |
