---
okf_version: "0.2"
type: Function
title: config_save
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
concept_id: src-tauri/src/config_ipc/config_save
language: rust
---

# config_save

[tauri::command]

## Signature

```rust
pub fn config_save(path: Option<String>, content: String) -> Result<(), String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 27–37 in `src-tauri/src/config_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config_ipc](/src-tauri/src/config_ipc.md) |
