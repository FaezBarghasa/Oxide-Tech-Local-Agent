---
okf_version: "0.2"
type: Function
title: hardware_flash_firmware
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
concept_id: src-tauri/src/hardware_ipc/hardware_flash_firmware
language: rust
---

# hardware_flash_firmware

[tauri::command]

## Signature

```rust
pub fn hardware_flash_firmware(
    request: FlashRequest,
) -> std::result::Result<FlashResult, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 98–102 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
| calls | [flash_firmware](/src-tauri/src/hardware_ipc/flash_firmware.md) |
