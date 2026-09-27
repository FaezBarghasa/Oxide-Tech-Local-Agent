---
okf_version: "0.2"
type: Class
title: BackgroundHealthMonitor
description: A background ticker that periodically tests endpoint health to allow zero-latency routing lookups.
resource: crates/oxide-gateway/src/probe.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/probe/BackgroundHealthMonitor
language: rust
---

# BackgroundHealthMonitor

A background ticker that periodically tests endpoint health to allow zero-latency routing lookups.

## Signature

```rust
pub struct BackgroundHealthMonitor
```

## Visibility

- `pub`

## Docstring

A background ticker that periodically tests endpoint health to allow zero-latency routing lookups.

## Methods

- `prober`
- `primary_reachable`
- `secondary_reachable`
- `primary_latency_ms`
- `secondary_latency_ms`

## Source
Lines 65–71 in `crates/oxide-gateway/src/probe.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [probe](/crates/oxide-gateway/src/probe.md) |
