---
okf_version: "0.2"
type: Function
title: run_gateway_server
resource: crates/oxide-gateway/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:34:00Z"
concept_id: crates/oxide-gateway/src/lib/run_gateway_server
language: rust
---

# run_gateway_server

## Signature

```rust
pub fn run_gateway_server(cfg: common::config::AppConfig) -> std::io::Result<()>
```

## Visibility

- `pub`

## Source
Lines 101–114 in `crates/oxide-gateway/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-gateway/src/lib.md) |
| calls | [run_gateway](/crates/oxide-gateway/src/lib/run_gateway.md) |
| called_by | [run_headless](/src-tauri/src/gateway_rt/run_headless.md) |
| called_by | [spawn_background](/src-tauri/src/gateway_rt/spawn_background.md) |
