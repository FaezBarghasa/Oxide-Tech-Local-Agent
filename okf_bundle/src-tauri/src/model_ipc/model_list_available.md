---
okf_version: "0.2"
type: Function
title: model_list_available
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
concept_id: src-tauri/src/model_ipc/model_list_available
language: rust
---

# model_list_available

[tauri::command]

## Signature

```rust
pub fn model_list_available() -> Result<ModelListResponse, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 181–276 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
| calls | [scan_default_local_gguf_models](/src-tauri/src/model_ipc/scan_default_local_gguf_models.md) |
| calls | [query_ollama_models](/src-tauri/src/model_ipc/query_ollama_models.md) |
| calls | [query_sglang_models](/src-tauri/src/model_ipc/query_sglang_models.md) |
