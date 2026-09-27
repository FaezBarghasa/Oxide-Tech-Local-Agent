---
okf_version: "0.2"
type: Function
title: drain_and_kill_inference
resource: crates/scheduler/src/gpu_resource_manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/gpu_resource_manager/drain_and_kill_inference_1
language: rust
---

# drain_and_kill_inference

## Signature

```rust
fn drain_and_kill_inference(&self) -> Result<(), anyhow::Error>
```

## Source
Lines 83–93 in `crates/scheduler/src/gpu_resource_manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpu_resource_manager](/crates/scheduler/src/gpu_resource_manager.md) |
