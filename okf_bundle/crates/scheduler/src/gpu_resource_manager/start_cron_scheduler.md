---
okf_version: "0.2"
type: Function
title: start_cron_scheduler
resource: crates/scheduler/src/gpu_resource_manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/gpu_resource_manager/start_cron_scheduler
language: rust
---

# start_cron_scheduler

## Signature

```rust
impl GpuResourceManager { pub fn start_cron_scheduler(self: Arc<Self>) -> Result<(), anyhow::Error> }
```

## Visibility

- `pub`

## Source
Lines 132–164 in `crates/scheduler/src/gpu_resource_manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpu_resource_manager](/crates/scheduler/src/gpu_resource_manager.md) |
