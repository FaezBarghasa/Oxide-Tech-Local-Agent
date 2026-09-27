---
okf_version: "0.2"
type: Function
title: hardware_get_chip_info
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
concept_id: src-tauri/src/hardware_ipc/hardware_get_chip_info
language: rust
---

# hardware_get_chip_info

[tauri::command]

## Signature

```rust
pub fn hardware_get_chip_info(
    device_identifier: String,
) -> std::result::Result<ChipInfoDto, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 91–95 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
| calls | [get_chip_info](/src-tauri/src/hardware_ipc/get_chip_info.md) |
