---
okf_version: "0.2"
type: Function
title: get_engine_matrix_status
description: "[tauri::command]"
resource: src-tauri/src/model_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:27:16Z"
concept_id: src-tauri/src/model_ipc/get_engine_matrix_status
language: rust
---

# get_engine_matrix_status

[tauri::command]

## Signature

```rust
pub fn get_engine_matrix_status() -> std::result::Result<Vec<EngineStatusEntry>, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 700–743 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
| calls | [probe_tcp_port](/src-tauri/src/model_ipc/probe_tcp_port.md) |
