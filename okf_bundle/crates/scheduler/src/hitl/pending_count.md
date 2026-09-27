---
okf_version: "0.2"
type: Function
title: pending_count
description: Check if there are active pending HITL requests.
resource: crates/scheduler/src/hitl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/hitl/pending_count
language: rust
---

# pending_count

Check if there are active pending HITL requests.

## Signature

```rust
impl HitlApprovalChannel { pub fn pending_count(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Check if there are active pending HITL requests.

## Source
Lines 102–104 in `crates/scheduler/src/hitl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hitl](/crates/scheduler/src/hitl.md) |
