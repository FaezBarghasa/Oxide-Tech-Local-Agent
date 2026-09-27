---
okf_version: "0.2"
type: Function
title: abort_saga
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/abort_saga
language: rust
---

# abort_saga

## Signature

```rust
impl DtxCoordinator { fn abort_saga(
        &self,
        dtx_id: DtxId,
        session: SessionKey,
        prepared: Vec<(ResourceRef, Arc<dyn ResourceTx>)>,
    ) }
```

## Source
Lines 157–181 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
