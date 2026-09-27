---
okf_version: "0.2"
type: Function
title: config_read
description: "[tauri::command]"
resource: src-tauri/src/config_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T05:57:34Z"
concept_id: src-tauri/src/config_ipc/config_read
language: rust
---

# config_read

[tauri::command]

## Signature

```rust
pub fn config_read(path: Option<String>) -> Result<ConfigFileDto, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 11–24 in `src-tauri/src/config_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config_ipc](/src-tauri/src/config_ipc.md) |
