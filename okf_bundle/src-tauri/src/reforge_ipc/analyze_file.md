---
okf_version: "0.2"
type: Function
title: analyze_file
resource: src-tauri/src/reforge_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/reforge_ipc/analyze_file
language: rust
---

# analyze_file

## Signature

```rust
pub fn analyze_file(req: ReforgeRequest) -> anyhow::Result<ReforgeResult>
```

## Visibility

- `pub`

## Source
Lines 87–255 in `src-tauri/src/reforge_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [reforge_ipc](/src-tauri/src/reforge_ipc.md) |
| calls | [pttx_entry](/src-tauri/src/reforge_ipc/pttx_entry.md) |
| called_by | [run_reforge_cli](/src-tauri/src/main/run_reforge_cli.md) |
| called_by | [reforge_analyze_file](/src-tauri/src/reforge_ipc/reforge_analyze_file.md) |
