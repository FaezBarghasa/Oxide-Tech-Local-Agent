---
okf_version: "0.2"
type: Function
title: switch_mode
resource: crates/scheduler/src/gpu_resource_manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/gpu_resource_manager/switch_mode
language: rust
---

# switch_mode

## Signature

```rust
impl GpuResourceManager { pub fn switch_mode(&self, new_mode: GpuMode) -> Result<(), anyhow::Error> }
```

## Visibility

- `pub`

## Source
Lines 29–81 in `crates/scheduler/src/gpu_resource_manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpu_resource_manager](/crates/scheduler/src/gpu_resource_manager.md) |
