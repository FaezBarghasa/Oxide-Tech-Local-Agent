---
okf_version: "0.2"
type: Function
title: get_status
description: Query the status of a distributed transaction
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/get_status_1
language: rust
---

# get_status

Query the status of a distributed transaction

## Signature

```rust
pub fn get_status(&self, dtx_id: DtxId) -> Option<DtxStatus>
```

## Visibility

- `pub`

## Docstring

Query the status of a distributed transaction

## Source
Lines 210–213 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
