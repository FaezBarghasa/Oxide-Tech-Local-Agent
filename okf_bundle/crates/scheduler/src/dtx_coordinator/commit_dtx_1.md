---
okf_version: "0.2"
type: Function
title: commit_dtx
description: Mark a distributed transaction as committed
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/commit_dtx_1
language: rust
---

# commit_dtx

Mark a distributed transaction as committed

## Signature

```rust
pub fn commit_dtx(&self, dtx_id: DtxId) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Mark a distributed transaction as committed

## Source
Lines 184–194 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
