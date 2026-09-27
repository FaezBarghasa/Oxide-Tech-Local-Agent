---
okf_version: "0.2"
type: Class
title: TelemetryConfig
description: Configuration for the telemetry subsystem.
resource: crates/telemetry/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:telemetry"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/telemetry/src/lib/TelemetryConfig
language: rust
---

# TelemetryConfig

Configuration for the telemetry subsystem.

## Signature

```rust
pub struct TelemetryConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Configuration for the telemetry subsystem.
[derive(Debug, Clone)]

## Methods

- `otlp_endpoint`
- `service_name`
- `log_filter`
- `json_logs`

## Source
Lines 13–22 in `crates/telemetry/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/telemetry/src/lib.md) |
