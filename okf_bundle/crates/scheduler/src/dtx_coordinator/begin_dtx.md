---
okf_version: "0.2"
type: Function
title: begin_dtx
description: Begin a new distributed transaction across multiple domains
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/begin_dtx
language: rust
---

# begin_dtx

Begin a new distributed transaction across multiple domains

## Signature

```rust
impl DtxCoordinator { pub fn begin_dtx(
        &self,
        title: &str,
        initiator: &str,
        domains: Vec<DomainTarget>,
    ) -> DtxId }
```

## Visibility

- `pub`

## Docstring

Begin a new distributed transaction across multiple domains

## Source
Lines 41–55 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
