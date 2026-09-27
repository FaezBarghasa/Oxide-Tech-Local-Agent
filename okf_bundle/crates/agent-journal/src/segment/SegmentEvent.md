---
okf_version: "0.2"
type: Class
title: SegmentEvent
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/agent-journal/src/segment.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/segment/SegmentEvent
language: rust
---

# SegmentEvent

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct SegmentEvent
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `seq`
- `dtx_id`
- `session_id`
- `at`
- `kind`
- `payload`
- `hash_prev`

## Source
Lines 29–37 in `crates/agent-journal/src/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/agent-journal/src/segment.md) |
