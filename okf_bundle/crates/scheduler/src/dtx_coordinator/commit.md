---
okf_version: "0.2"
type: Function
title: commit
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/commit
language: rust
---

# commit

## Signature

```rust
impl MockTransactionalResource { fn commit(&self, _env: &DtxEnvelope) -> Result<(), String> }
```

## Source
Lines 234–236 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
