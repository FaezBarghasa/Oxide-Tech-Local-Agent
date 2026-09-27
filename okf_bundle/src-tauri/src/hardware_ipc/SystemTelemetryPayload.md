---
okf_version: "0.2"
type: Class
title: SystemTelemetryPayload
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: src-tauri/src/hardware_ipc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:19:13Z"
concept_id: src-tauri/src/hardware_ipc/SystemTelemetryPayload
language: rust
---

# SystemTelemetryPayload

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct SystemTelemetryPayload
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`
- `serde(rename_all = "camelCase")`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]
[serde(rename_all = "camelCase")]

## Methods

- `gpu_mode`
- `vram_used_bytes`
- `vram_total_bytes`
- `vram_temperature_c`
- `ram_available_bytes`
- `ram_total_bytes`
- `cpu_load_percent`
- `active_processes_count`
- `daemon_status`

## Source
Lines 68–78 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
