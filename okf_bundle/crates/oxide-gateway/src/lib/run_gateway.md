---
okf_version: "0.2"
type: Function
title: run_gateway
resource: crates/oxide-gateway/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:34:00Z"
concept_id: crates/oxide-gateway/src/lib/run_gateway
language: rust
---

# run_gateway

## Signature

```rust
pub fn run_gateway(state: Arc<AppState>, host: &str, port: u16) -> std::io::Result<()>
```

## Visibility

- `pub`

## Source
Lines 51–99 in `crates/oxide-gateway/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-gateway/src/lib.md) |
| called_by | [run_gateway_server](/crates/oxide-gateway/src/lib/run_gateway_server.md) |
