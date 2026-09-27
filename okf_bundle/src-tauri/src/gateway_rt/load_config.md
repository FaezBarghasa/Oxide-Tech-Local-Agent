---
okf_version: "0.2"
type: Function
title: load_config
description: Load config from an explicit path or fall back to built-in defaults.
resource: src-tauri/src/gateway_rt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T01:22:10Z"
concept_id: src-tauri/src/gateway_rt/load_config
language: rust
---

# load_config

Load config from an explicit path or fall back to built-in defaults.

## Signature

```rust
pub fn load_config(config_path: Option<&str>) -> AppConfig
```

## Visibility

- `pub`

## Docstring

Load config from an explicit path or fall back to built-in defaults.

## Source
Lines 12–27 in `src-tauri/src/gateway_rt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gateway_rt](/src-tauri/src/gateway_rt.md) |
| called_by | [run_headless](/src-tauri/src/gateway_rt/run_headless.md) |
| called_by | [spawn_background](/src-tauri/src/gateway_rt/spawn_background.md) |
