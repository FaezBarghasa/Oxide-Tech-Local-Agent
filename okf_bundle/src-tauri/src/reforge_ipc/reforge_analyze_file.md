---
okf_version: "0.2"
type: Function
title: reforge_analyze_file
description: "[tauri::command]"
resource: src-tauri/src/reforge_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/reforge_ipc/reforge_analyze_file
language: rust
---

# reforge_analyze_file

[tauri::command]

## Signature

```rust
pub fn reforge_analyze_file(request: ReforgeRequest) -> Result<ReforgeResult, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 262–267 in `src-tauri/src/reforge_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [reforge_ipc](/src-tauri/src/reforge_ipc.md) |
| calls | [analyze_file](/src-tauri/src/reforge_ipc/analyze_file.md) |
