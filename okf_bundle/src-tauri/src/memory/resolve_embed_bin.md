---
okf_version: "0.2"
type: Function
title: resolve_embed_bin
description: "Resolve the `oxide-embed` binary. Returns the path to execute and a flag"
resource: src-tauri/src/memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/memory/resolve_embed_bin
language: rust
---

# resolve_embed_bin

Resolve the `oxide-embed` binary. Returns the path to execute and a flag

## Signature

```rust
pub fn resolve_embed_bin() -> Result<PathBuf, String>
```

## Visibility

- `pub`

## Docstring

Resolve the `oxide-embed` binary. Returns the path to execute and a flag
telling whether it was found on `PATH` (bare command) or as a file.

## Source
Lines 53–73 in `src-tauri/src/memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [memory](/src-tauri/src/memory.md) |
| calls | [candidate_paths](/src-tauri/src/memory/candidate_paths.md) |
| called_by | [memory_env](/src-tauri/src/memory/memory_env.md) |
| called_by | [run_embed](/src-tauri/src/memory/run_embed.md) |
| called_by | [run_embed_blocking](/src-tauri/src/memory/run_embed_blocking.md) |
