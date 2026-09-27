---
okf_version: "0.2"
type: Function
title: register_resource
description: Register a participating transactional resource
resource: crates/scheduler/src/dtx_coordinator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/scheduler/src/dtx_coordinator/register_resource_1
language: rust
---

# register_resource

Register a participating transactional resource

## Signature

```rust
pub fn register_resource(&self, rref: ResourceRef, resource: Arc<dyn ResourceTx>)
```

## Visibility

- `pub`

## Docstring

Register a participating transactional resource

## Source
Lines 35–38 in `crates/scheduler/src/dtx_coordinator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx_coordinator](/crates/scheduler/src/dtx_coordinator.md) |
