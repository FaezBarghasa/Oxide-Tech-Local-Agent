---
okf_version: "0.2"
type: Function
title: init
description: Initialise the global tracing subscriber (with optional OTLP export).
resource: crates/telemetry/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:telemetry"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/telemetry/src/lib/init_1
language: rust
---

# init

Initialise the global tracing subscriber (with optional OTLP export).

## Signature

```rust
pub fn init(self) -> Result<TelemetryGuard>
```

## Visibility

- `pub`

## Docstring

Initialise the global tracing subscriber (with optional OTLP export).

Returns a `TelemetryGuard` — dropping it flushes all pending spans and logs.

## Source
Lines 54–108 in `crates/telemetry/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/telemetry/src/lib.md) |
