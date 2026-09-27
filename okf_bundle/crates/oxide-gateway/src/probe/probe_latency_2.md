---
okf_version: "0.2"
type: Function
title: probe_latency
description: Backwards compatible functions using the pooled prober
resource: crates/oxide-gateway/src/probe.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/probe/probe_latency_2
language: rust
---

# probe_latency

Backwards compatible functions using the pooled prober

## Signature

```rust
pub fn probe_latency(base_url: &str) -> Result<Duration, anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Backwards compatible functions using the pooled prober

## Source
Lines 163–165 in `crates/oxide-gateway/src/probe.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [probe](/crates/oxide-gateway/src/probe.md) |
| calls | [get_prober](/crates/oxide-gateway/src/probe/get_prober.md) |
