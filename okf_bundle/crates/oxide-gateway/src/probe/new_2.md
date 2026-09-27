---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-gateway/src/probe.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/probe/new_2
language: rust
---

# new

## Signature

```rust
impl BackgroundHealthMonitor { pub fn new(
        primary_url: String,
        secondary_url: String,
        threshold_ms: u64,
        check_interval: Duration,
    ) -> (Self, tokio::task::JoinHandle<()>) }
```

## Visibility

- `pub`

## Source
Lines 74–131 in `crates/oxide-gateway/src/probe.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [probe](/crates/oxide-gateway/src/probe.md) |
