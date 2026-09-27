---
okf_version: "0.2"
type: Function
title: execute_saga
description: "Execute a full Saga transaction with pre-flight irreversible check, compensation snapshotting,"
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/execute_saga
language: rust
---

# execute_saga

Execute a full Saga transaction with pre-flight irreversible check, compensation snapshotting,

## Signature

```rust
impl DtxCoordinator { pub fn execute_saga(
        &self,
        dtx_id: DtxId,
        session: SessionKey,
        participants: Vec<ResourceRef>,
    ) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Execute a full Saga transaction with pre-flight irreversible check, compensation snapshotting,
2-phase prepare, commit, and atomic rollback on failure.

## Source
Lines 59–155 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
