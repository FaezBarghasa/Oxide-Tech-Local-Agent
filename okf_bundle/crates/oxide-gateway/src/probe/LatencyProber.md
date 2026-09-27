---
okf_version: "0.2"
type: Class
title: LatencyProber
description: "Connection-pooled latency prober that avoids allocating a new reqwest::Client on each call."
resource: crates/oxide-gateway/src/probe.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/probe/LatencyProber
language: rust
---

# LatencyProber

Connection-pooled latency prober that avoids allocating a new reqwest::Client on each call.

## Signature

```rust
pub struct LatencyProber
```

## Visibility

- `pub`

## Docstring

Connection-pooled latency prober that avoids allocating a new reqwest::Client on each call.

## Methods

- `client`

## Source
Lines 8–10 in `crates/oxide-gateway/src/probe.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [probe](/crates/oxide-gateway/src/probe.md) |
