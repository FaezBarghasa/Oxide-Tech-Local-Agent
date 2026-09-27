---
okf_version: "0.2"
type: Function
title: memory_search
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
concept_id: src-tauri/src/memory/memory_search
language: rust
---

# memory_search

[tauri::command]

## Signature

```rust
pub fn memory_search(
    cwd: Option<String>,
    query: String,
    stair: Option<bool>,
    limit: Option<usize>,
    budget: Option<usize>,
    with_graph: Option<bool>,
) -> Result<EmbedResult, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 172–197 in `src-tauri/src/memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [memory](/src-tauri/src/memory.md) |
| calls | [resolve_cwd](/src-tauri/src/memory/resolve_cwd.md) |
| calls | [run_embed](/src-tauri/src/memory/run_embed.md) |
