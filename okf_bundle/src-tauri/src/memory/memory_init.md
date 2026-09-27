---
okf_version: "0.2"
type: Function
title: memory_init
description: "[tauri::command]"
resource: src-tauri/src/memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/memory/memory_init
language: rust
---

# memory_init

[tauri::command]

## Signature

```rust
pub fn memory_init(cwd: Option<String>, name: Option<String>) -> Result<EmbedResult, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 149–159 in `src-tauri/src/memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [memory](/src-tauri/src/memory.md) |
| calls | [resolve_cwd](/src-tauri/src/memory/resolve_cwd.md) |
| calls | [run_embed](/src-tauri/src/memory/run_embed.md) |
