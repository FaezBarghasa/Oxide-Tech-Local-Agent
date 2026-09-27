---
okf_version: "0.2"
type: Class
title: TelemetryGuard
description: "RAII guard — when dropped, shuts down the OpenTelemetry tracer provider."
resource: crates/telemetry/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:telemetry"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/telemetry/src/lib/TelemetryGuard
language: rust
---

# TelemetryGuard

RAII guard — when dropped, shuts down the OpenTelemetry tracer provider.

## Signature

```rust
pub struct TelemetryGuard
```

## Visibility

- `pub`

## Docstring

RAII guard — when dropped, shuts down the OpenTelemetry tracer provider.

## Methods

- `otel_active`

## Source
Lines 112–114 in `crates/telemetry/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/telemetry/src/lib.md) |
