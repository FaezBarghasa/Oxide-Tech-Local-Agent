---
okf_version: "0.2"
type: Module
title: gateway_rt
description: Embedded gateway runtime.
resource: src-tauri/src/gateway_rt.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T01:22:10Z"
concept_id: src-tauri/src/gateway_rt
language: rust
---

# gateway_rt

Embedded gateway runtime.

## Docstring

Embedded gateway runtime.

The desktop binary runs the full stack in-process:
- `daemon` subcommand  → gateway in the foreground (systemd: `oxide-agent daemon`)
- desktop window mode   → gateway on a background thread, WebView in front

Mirrors `workspace/gateway/src/main.rs` threading and config behaviour.

## Relationships

| Type | Target |
|------|--------|
| related | [load_config](/src-tauri/src/gateway_rt/load_config.md) |
| related | [run_headless](/src-tauri/src/gateway_rt/run_headless.md) |
| related | [spawn_background](/src-tauri/src/gateway_rt/spawn_background.md) |
| related | [probe_gateway](/src-tauri/src/gateway_rt/probe_gateway.md) |
