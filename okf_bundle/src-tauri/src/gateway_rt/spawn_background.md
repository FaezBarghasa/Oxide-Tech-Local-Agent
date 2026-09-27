---
okf_version: "0.2"
type: Function
title: spawn_background
description: Spawn the gateway on a background thread; returns immediately.
resource: src-tauri/src/gateway_rt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T01:22:10Z"
concept_id: src-tauri/src/gateway_rt/spawn_background
language: rust
---

# spawn_background

Spawn the gateway on a background thread; returns immediately.

## Signature

```rust
pub fn spawn_background(config_path: Option<String>)
```

## Visibility

- `pub`

## Docstring

Spawn the gateway on a background thread; returns immediately.
The desktop WebView then talks to it at `http://127.0.0.1:8080`.

## Source
Lines 44–82 in `src-tauri/src/gateway_rt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gateway_rt](/src-tauri/src/gateway_rt.md) |
| calls | [probe_gateway](/src-tauri/src/gateway_rt/probe_gateway.md) |
| calls | [load_config](/src-tauri/src/gateway_rt/load_config.md) |
| calls | [run_gateway_server](/crates/oxide-gateway/src/lib/run_gateway_server.md) |
| called_by | [run_desktop](/src-tauri/src/main/run_desktop.md) |
