---
okf_version: "0.2"
type: Function
title: run_embed_blocking
description: Synchronous variant for CLI subcommands (doctor / memory passthrough).
resource: src-tauri/src/memory.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/memory/run_embed_blocking
language: rust
---

# run_embed_blocking

Synchronous variant for CLI subcommands (doctor / memory passthrough).

## Signature

```rust
pub fn run_embed_blocking(args: &[String], cwd: &Path) -> Result<std::process::Output, String>
```

## Visibility

- `pub`

## Docstring

Synchronous variant for CLI subcommands (doctor / memory passthrough).

## Source
Lines 106–113 in `src-tauri/src/memory.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [memory](/src-tauri/src/memory.md) |
| calls | [resolve_embed_bin](/src-tauri/src/memory/resolve_embed_bin.md) |
| called_by | [run_memory_passthrough](/src-tauri/src/main/run_memory_passthrough.md) |
