---
okf_version: "0.2"
type: Function
title: from_env
description: "Build configuration from environment variables:"
resource: crates/telemetry/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:telemetry"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/telemetry/src/lib/from_env
language: rust
---

# from_env

Build configuration from environment variables:

## Signature

```rust
impl TelemetryConfig { pub fn from_env() -> Self }
```

## Visibility

- `pub`

## Docstring

Build configuration from environment variables:
- `OTLP_ENDPOINT` — OTLP gRPC collector URL
- `OTEL_SERVICE_NAME` — service name label
- `RUST_LOG` — log filter (defaults to `info`)
- `LOG_JSON=true` — emit JSON-formatted logs

## Source
Lines 41–49 in `crates/telemetry/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/telemetry/src/lib.md) |
