---
okf_version: "0.2"
type: Function
title: get_chip_info
resource: src-tauri/src/hardware_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:19:13Z"
concept_id: src-tauri/src/hardware_ipc/get_chip_info
language: rust
---

# get_chip_info

## Signature

```rust
pub fn get_chip_info(device_identifier: String) -> Result<ChipInfoDto>
```

## Visibility

- `pub`

## Source
Lines 204–291 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
| called_by | [hardware_get_chip_info](/src-tauri/src/hardware_ipc/hardware_get_chip_info.md) |
