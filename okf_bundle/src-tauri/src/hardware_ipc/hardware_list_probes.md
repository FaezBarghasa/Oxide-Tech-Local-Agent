---
okf_version: "0.2"
type: Function
title: hardware_list_probes
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
concept_id: src-tauri/src/hardware_ipc/hardware_list_probes
language: rust
---

# hardware_list_probes

[tauri::command]

## Signature

```rust
pub fn hardware_list_probes() -> std::result::Result<ProbeDevicesResult, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 86–88 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
| calls | [list_probe_devices](/src-tauri/src/hardware_ipc/list_probe_devices.md) |
