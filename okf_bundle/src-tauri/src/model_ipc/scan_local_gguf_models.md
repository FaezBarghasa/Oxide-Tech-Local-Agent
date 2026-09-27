---
okf_version: "0.2"
type: Function
title: scan_local_gguf_models
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
concept_id: src-tauri/src/model_ipc/scan_local_gguf_models
language: rust
---

# scan_local_gguf_models

[tauri::command]

## Signature

```rust
pub fn scan_local_gguf_models(
    custom_paths: Vec<String>,
) -> std::result::Result<Vec<DiscoveredGgufModel>, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 661–697 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
| calls | [inspect_gguf_file](/src-tauri/src/model_ipc/inspect_gguf_file.md) |
