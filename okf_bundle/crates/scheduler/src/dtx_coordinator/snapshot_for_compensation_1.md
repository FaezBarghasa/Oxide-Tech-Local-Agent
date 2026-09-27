---
okf_version: "0.2"
type: Function
title: snapshot_for_compensation
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/snapshot_for_compensation_1
language: rust
---

# snapshot_for_compensation

## Signature

```rust
fn snapshot_for_compensation(
            &self,
            _env: &DtxEnvelope,
        ) -> Result<ArtifactRef, String>
```

## Source
Lines 243–252 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
