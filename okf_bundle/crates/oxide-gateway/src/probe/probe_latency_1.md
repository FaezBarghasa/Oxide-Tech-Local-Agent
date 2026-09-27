---
okf_version: "0.2"
type: Function
title: probe_latency
description: Probe the online API endpoint for reachability and measure round-trip latency.
resource: crates/oxide-gateway/src/probe.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/probe/probe_latency_1
language: rust
---

# probe_latency

Probe the online API endpoint for reachability and measure round-trip latency.

## Signature

```rust
pub fn probe_latency(&self, base_url: &str) -> Result<Duration, anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Probe the online API endpoint for reachability and measure round-trip latency.

## Source
Lines 32–52 in `crates/oxide-gateway/src/probe.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [probe](/crates/oxide-gateway/src/probe.md) |
