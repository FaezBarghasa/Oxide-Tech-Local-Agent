---
okf_version: "0.2"
type: Function
title: model_run_prompt
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
concept_id: src-tauri/src/model_ipc/model_run_prompt
language: rust
---

# model_run_prompt

[tauri::command]

## Signature

```rust
pub fn model_run_prompt(req: RunPromptRequest) -> Result<RunPromptResponse, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 279–594 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
