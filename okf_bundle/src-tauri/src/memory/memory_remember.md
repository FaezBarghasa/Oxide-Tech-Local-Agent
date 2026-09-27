---
okf_version: "0.2"
type: Function
title: memory_remember
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
concept_id: src-tauri/src/memory/memory_remember
language: rust
---

# memory_remember

[tauri::command]

## Signature

```rust
pub fn memory_remember(
    cwd: Option<String>,
    content: String,
    kind: Option<String>,
    tags: Option<String>,
    symbol: Option<String>,
    auto_resolve: Option<bool>,
) -> Result<EmbedResult, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 215–247 in `src-tauri/src/memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [memory](/src-tauri/src/memory.md) |
| calls | [resolve_cwd](/src-tauri/src/memory/resolve_cwd.md) |
| calls | [run_embed](/src-tauri/src/memory/run_embed.md) |
