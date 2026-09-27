---
okf_version: "0.2"
type: Function
title: list_probe_devices
resource: src-tauri/src/hardware_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:19:13Z"
concept_id: src-tauri/src/hardware_ipc/list_probe_devices
language: rust
---

# list_probe_devices

## Signature

```rust
pub fn list_probe_devices() -> Result<ProbeDevicesResult>
```

## Visibility

- `pub`

## Source
Lines 169–202 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
| called_by | [hardware_list_probes](/src-tauri/src/hardware_ipc/hardware_list_probes.md) |
