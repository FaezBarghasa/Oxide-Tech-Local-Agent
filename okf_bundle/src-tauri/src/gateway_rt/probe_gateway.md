---
okf_version: "0.2"
type: Function
title: probe_gateway
description: "Minimal gateway liveness probe used by `doctor` and the UI status command."
resource: src-tauri/src/gateway_rt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T01:22:10Z"
concept_id: src-tauri/src/gateway_rt/probe_gateway
language: rust
---

# probe_gateway

Minimal gateway liveness probe used by `doctor` and the UI status command.

## Signature

```rust
pub fn probe_gateway(base_url: &str, timeout_secs: u64) -> Result<u16, String>
```

## Visibility

- `pub`

## Docstring

Minimal gateway liveness probe used by `doctor` and the UI status command.

## Source
Lines 85–97 in `src-tauri/src/gateway_rt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gateway_rt](/src-tauri/src/gateway_rt.md) |
| called_by | [spawn_background](/src-tauri/src/gateway_rt/spawn_background.md) |
| called_by | [gateway_status](/src-tauri/src/main/gateway_status.md) |
| called_by | [run_status](/src-tauri/src/main/run_status.md) |
