---
okf_version: "0.2"
type: Function
title: memory_env
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
concept_id: src-tauri/src/memory/memory_env
language: rust
---

# memory_env

[tauri::command]

## Signature

```rust
pub fn memory_env(cwd: Option<String>) -> Result<MemoryEnv, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 118–140 in `src-tauri/src/memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [memory](/src-tauri/src/memory.md) |
| calls | [resolve_embed_bin](/src-tauri/src/memory/resolve_embed_bin.md) |
| calls | [resolve_cwd](/src-tauri/src/memory/resolve_cwd.md) |
