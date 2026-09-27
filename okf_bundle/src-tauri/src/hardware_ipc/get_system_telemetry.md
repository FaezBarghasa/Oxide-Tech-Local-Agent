---
okf_version: "0.2"
type: Function
title: get_system_telemetry
description: "[tauri::command]"
resource: src-tauri/src/hardware_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:19:13Z"
concept_id: src-tauri/src/hardware_ipc/get_system_telemetry
language: rust
---

# get_system_telemetry

[tauri::command]

## Signature

```rust
pub fn get_system_telemetry() -> std::result::Result<SystemTelemetryPayload, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 81–83 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
| calls | [fetch_real_system_telemetry](/src-tauri/src/hardware_ipc/fetch_real_system_telemetry.md) |
