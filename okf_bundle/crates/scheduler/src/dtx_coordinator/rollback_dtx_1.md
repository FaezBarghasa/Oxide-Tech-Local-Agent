---
okf_version: "0.2"
type: Function
title: rollback_dtx
description: Broadcast rollback across all domain apps for a failed multi-domain task
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/rollback_dtx_1
language: rust
---

# rollback_dtx

Broadcast rollback across all domain apps for a failed multi-domain task

## Signature

```rust
pub fn rollback_dtx(&self, dtx_id: DtxId, reason: &str) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Broadcast rollback across all domain apps for a failed multi-domain task

## Source
Lines 197–207 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
