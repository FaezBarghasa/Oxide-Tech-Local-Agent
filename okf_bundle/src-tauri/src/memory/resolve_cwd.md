---
okf_version: "0.2"
type: Function
title: resolve_cwd
resource: src-tauri/src/memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/memory/resolve_cwd
language: rust
---

# resolve_cwd

## Signature

```rust
pub fn resolve_cwd(cwd: Option<String>) -> PathBuf
```

## Visibility

- `pub`

## Source
Lines 75–88 in `src-tauri/src/memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [memory](/src-tauri/src/memory.md) |
| called_by | [memory_conflicts](/src-tauri/src/memory/memory_conflicts.md) |
| called_by | [memory_context](/src-tauri/src/memory/memory_context.md) |
| called_by | [memory_env](/src-tauri/src/memory/memory_env.md) |
| called_by | [memory_explain](/src-tauri/src/memory/memory_explain.md) |
| called_by | [memory_index](/src-tauri/src/memory/memory_index.md) |
| called_by | [memory_init](/src-tauri/src/memory/memory_init.md) |
| called_by | [memory_recall](/src-tauri/src/memory/memory_recall.md) |
| called_by | [memory_remember](/src-tauri/src/memory/memory_remember.md) |
| called_by | [memory_search](/src-tauri/src/memory/memory_search.md) |
| called_by | [memory_status](/src-tauri/src/memory/memory_status.md) |
