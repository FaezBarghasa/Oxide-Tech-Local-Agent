---
okf_version: "0.2"
type: Module
title: memory
description: oxide-embed memory bridge.
resource: src-tauri/src/memory.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/memory
language: rust
---

# memory

oxide-embed memory bridge.

## Docstring

oxide-embed memory bridge.

All project memory flows through the `oxide-embed` binary:
init / index / search / context / remember / recall / conflicts / explain.

Binary resolution order (first hit wins):
1. `OXIDE_EMBED_BIN` env var (explicit override, used by dev shells)
2. `<current-exe-dir>/oxide-embed` (Tauri `externalBin` sidecar staging)
3. `/usr/lib/oxide-agent/oxide-embed` (deb install layout)
4. `/usr/bin/oxide-embed`
5. `oxide-embed` on `PATH` (dev default, e.g. `~/.local/bin/oxide-embed`)

## Relationships

| Type | Target |
|------|--------|
| related | [MemoryEnv](/src-tauri/src/memory/MemoryEnv.md) |
| related | [EmbedResult](/src-tauri/src/memory/EmbedResult.md) |
| related | [candidate_paths](/src-tauri/src/memory/candidate_paths.md) |
| related | [resolve_embed_bin](/src-tauri/src/memory/resolve_embed_bin.md) |
| related | [resolve_cwd](/src-tauri/src/memory/resolve_cwd.md) |
| related | [run_embed](/src-tauri/src/memory/run_embed.md) |
| related | [run_embed_blocking](/src-tauri/src/memory/run_embed_blocking.md) |
| related | [memory_env](/src-tauri/src/memory/memory_env.md) |
| related | [memory_status](/src-tauri/src/memory/memory_status.md) |
| related | [memory_init](/src-tauri/src/memory/memory_init.md) |
| related | [memory_index](/src-tauri/src/memory/memory_index.md) |
| related | [memory_search](/src-tauri/src/memory/memory_search.md) |
| related | [memory_context](/src-tauri/src/memory/memory_context.md) |
| related | [memory_remember](/src-tauri/src/memory/memory_remember.md) |
| related | [memory_recall](/src-tauri/src/memory/memory_recall.md) |
| related | [memory_explain](/src-tauri/src/memory/memory_explain.md) |
| related | [memory_conflicts](/src-tauri/src/memory/memory_conflicts.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
