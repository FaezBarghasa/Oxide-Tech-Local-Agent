---
okf_version: "0.2"
type: Function
title: flash_firmware
resource: src-tauri/src/hardware_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:19:13Z"
concept_id: src-tauri/src/hardware_ipc/flash_firmware
language: rust
---

# flash_firmware

## Signature

```rust
pub fn flash_firmware(request: FlashRequest) -> Result<FlashResult>
```

## Visibility

- `pub`

## Source
Lines 293–333 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
| called_by | [hardware_flash_firmware](/src-tauri/src/hardware_ipc/hardware_flash_firmware.md) |
