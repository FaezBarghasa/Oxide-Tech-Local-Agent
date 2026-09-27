---
okf_version: "0.2"
type: Function
title: run_headless
description: Run the gateway on the calling thread (foreground / headless service mode).
resource: src-tauri/src/gateway_rt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T01:22:10Z"
concept_id: src-tauri/src/gateway_rt/run_headless
language: rust
---

# run_headless

Run the gateway on the calling thread (foreground / headless service mode).

## Signature

```rust
pub fn run_headless(config_path: Option<&str>) -> anyhow::Result<()>
```

## Visibility

- `pub`

## Docstring

Run the gateway on the calling thread (foreground / headless service mode).

## Source
Lines 30–40 in `src-tauri/src/gateway_rt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gateway_rt](/src-tauri/src/gateway_rt.md) |
| calls | [load_config](/src-tauri/src/gateway_rt/load_config.md) |
| calls | [run_gateway_server](/crates/oxide-gateway/src/lib/run_gateway_server.md) |
| called_by | [main](/src-tauri/src/main/main.md) |
